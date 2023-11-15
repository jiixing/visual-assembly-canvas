import { Handle, Position, NodeProps } from "reactflow"

import { MachineValueViewer } from "./MachineValueViewer"

import { MachineEditor } from "../../../editor/Editor"
import { MachineProps } from "../../../types/blocks"

export function MachineBlock(props: NodeProps<MachineProps>) {
  const { data } = props

  return (
    <div className="font-mono bg-slate-1 relative group">
      <Handle
        type="target"
        position={Position.Left}
        id="0"
        className="bg-crimson-9 group-hover:bg-cyan-11 hover:!bg-gray-12 hover:border-crimson-9 px-1 py-1 ml-[-1px] border-2 z-10"
      ></Handle>

      <div className="px-3 py-3 border-2 rounded-2 hover:border-cyan-11">
        <div className="flex flex-col space-y-2 text-gray-50">
          <div className="min-h-[100px]">
            <div className="nodrag">
              <MachineEditor {...data} />
            </div>
          </div>

          <MachineValueViewer id={data.id} />
        </div>
      </div>

      <Handle
        type="source"
        position={Position.Right}
        id="1"
        className="bg-crimson-9 group-hover:bg-cyan-11 hover:!bg-gray-12 hover:border-crimson-9 px-1 py-1 mr-[-1px] border-2 z-10"
      />
    </div>
  )
}