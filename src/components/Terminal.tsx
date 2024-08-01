import { DebugStatus, useAssemblySource } from "@/context/AssemblyContext";
import { KeyboardEvent, useRef } from "react";

export default function Terminal() {
    const { std_out, push_std_out, debug_status, set_debug_status } = useAssemblySource();
    const input_text = useRef<string>('');
    const addLineBreak = (str: string) => (
        str.split('\n').map((subStr, i, lines) => {
            return (
                <>
                    {subStr}
                    {(i == lines.length - 1) ? null : <br />}
                </>
            );
        })
    );
    const handleEnter = (ev: KeyboardEvent) => {
        if (ev.key === 'Enter' && (debug_status === DebugStatus.INPUT_CONTINUE || debug_status === DebugStatus.INPUT_STEP)) {
            push_std_out('text', input_text.current);
        }
    };
    return (
        <div id="terminal" className="p-2 bg-zinc-700">
            {/* Standard Output */}
            <div id="terminal-output" className="p-4 bg-primary text-sm text-gray-300 overflow-scroll">
                {std_out.map((out, i) => {
                    switch (out.type) {
                        case "compile":
                            return <p key={i} className="text-yellow-500 font-bold">{out.message}</p>
                        case "run":
                            return <p key={i} className="text-green-500 font-bold">{out.message}</p>
                        case "error":
                            return <p key={i}><span className=" text-red-500 font-bold">Error: </span>{out.message}</p>
                        case "red":
                            return <p key={i} className="text-red-500 font-bold">{out.message}</p>
                        default:
                            return <p key={i}>{addLineBreak(out.message)}</p>
                    }
                })}
            </div>
            {/* Standard Input */}
            <div id="terminal-input" className="py-1 px-6 bg-zinc-600">
                <span className="text-gray bg-white inline-block pl-2 pr-3 py-1">{">>"}</span>
                <input type="text" onKeyUp={handleEnter} onChange={e => input_text.current = e.target.value} placeholder="enter user input...." className="px-2 py-1 w-2/5 bg-slate-100 focus:bg-white" />
            </div>
        </div>
    )
}