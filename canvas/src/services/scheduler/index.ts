import { manager } from "../../core"
import { audioManager } from "../audio/manager"

import { $status } from "../../store/status"
import { $delay } from "../../store/canvas"

const handlers = {
  start: async () => {
    await audioManager.ready()
    manager.onRunStart()
  },

  stop: async () => {
    manager.onRunCleanup()
  },
} as const

const updaters = {
  canvas() {
    // TODO: implement tickBatch() to the WebAssembly API to batch ticks for performance in large programs.
    const ok = manager.tickOnce()
    if (!ok) scheduler.pause()
  },

  effect: () => manager.performSideEffects(),
  blocks: () => manager.syncBlocks(),
  machine: () => manager.syncMachineState(),
  highlight: () => manager.highlight(),
} as const

type Updater = keyof typeof updaters

type Timer = ReturnType<typeof setTimeout>

export class Scheduler {
  /** Request ID of the current animation frame. */
  frameRequestId = 0

  frame = 0

  timers: Partial<Record<Updater, Timer>> = {}

  constructor() {
    this.setup().then()
  }

  get running() {
    return $status.value?.running ?? false
  }

  set running(value: boolean) {
    $status.setKey("running", value)
  }

  async setup() {}

  public start = async () => {
    this.running = true

    // Start the run.
    await handlers.start()

    // Begin requesting animation frame.
    this.frame = 0
    this.frameRequestId = requestAnimationFrame(this.render)

    // Engine is ticked as fast as possible, given the user-defined delay.
    this.loop("canvas", $delay.get())

    // Side effects such as MIDI and synths are processed every 5ms.
    this.loop("effect", 5)
  }

  private loop(type: Updater, delay: number) {
    const fn = updaters[type]
    this.timers[type] = setInterval(fn, delay)
  }

  private clearTimers(...updaters: Updater[]) {
    updaters.forEach((updater) => clearInterval(this.timers[updater]))
  }

  public pause = () => {
    this.running = false
    this.clearTimers("canvas", "effect", "highlight", "machine")
    cancelAnimationFrame(this.frameRequestId)
  }

  /** The render loop updates the UI. */
  private render = () => {
    // Abort the render loop if the scheduler is not running.
    if (!this.running) return cancelAnimationFrame(this.frameRequestId)

    // Increment the frame counter.
    this.frame++

    // Render blocks at maximum FPS.
    // TODO: adaptive FPS based on canvas heuristics.
    updaters.blocks()

    if (this.every(20)) updaters.highlight()
    if (this.every(10)) updaters.machine()

    this.frameRequestId = requestAnimationFrame(this.render)
  }

  every(frame: number) {
    return this.frame % frame === 0
  }
}

export const scheduler = new Scheduler()