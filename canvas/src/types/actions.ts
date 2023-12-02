import { Waveform } from "./waveform"
import { MidiInputEvent, MidiOutputFormat, PixelMode } from "./enums"

export interface MidiAction {
  Midi: {
    event: MidiInputEvent
    note: number
    value: number
    channel: number
    port: number
  }
}

export type Action =
  | { Data: { body: number[] } }
  | { Read: { address: number; count: number } }
  | { Write: { address: number; data: number[] } }
  | { Reset: null }
  | MidiAction
  | { SetMidiPort: { port: number } }
  | { SetMidiChannels: { channels: number[] } }
  | { SetMidiInputEvent: { event: MidiInputEvent } }
  | { SetMidiOutputFormat: { format: MidiOutputFormat } }
  | { SetWaveform: { waveform: Waveform } }
  | { SetPixelMode: { mode: PixelMode } }
  | { SetAutoReset: { auto_reset: boolean } }
