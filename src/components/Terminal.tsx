import { DebugStatus, InputStatus, useAssemblySource } from "@/context/AssemblyContext";
import { useSource } from "@/context/SourceContext";
import { debug_continue, debug_step, handleRun } from "@/helpers/control";
import { Fragment, KeyboardEvent, useRef } from "react";

export default function Terminal() {
    const source = useSource();
    const ass_source = useAssemblySource();
    const { std_out, push_std_out, input_status, debug_status, toolbar_btn } = ass_source;
    const input_text = useRef<string>('');
    const addLineBreak = (str: string) => (
        str.split('\n').map((subStr, i, lines) => {
            return (
                <Fragment key={subStr + i}>
                    {subStr}
                    {(i == lines.length - 1) ? null : <br />}
                </Fragment>
            );
        })
    );
    const handleEnter = (ev: KeyboardEvent) => {
        if (ev.key !== 'Enter' || input_status.current === InputStatus.None)
            return;

        let input: number = -1;
        let err: string | undefined = undefined;

        // validate input
        if (input_status.current === InputStatus.GetChar) {
            if (input_text.current.length > 1)
                err = "Too many characters detected.";
            else if (input_text.current.length == 0)
                err = "No characters detected.";
            else
                input = input_text.current.charCodeAt(0);
        } else if (input_status.current === InputStatus.GetNumber) {
            const number = parseInt(input_text.current, 10);
            if (Number.isNaN(number))
                err = "The text could not be converted to a valid number.";
            else
                input = number;
        }
        if (err)
            push_std_out('text', err);
        else {
            input_status.current = InputStatus.None;
            switch (debug_status) {
                case DebugStatus.RUNNING:
                    handleRun(ass_source, input);
                    break;
                case DebugStatus.CONTINUE:
                    debug_continue(source, ass_source, input);
                    break;
                case DebugStatus.STEP:
                    debug_step(source, ass_source, input);
            }
        }
        // push_std_out('text', input_text.current); // For debugging, comment after use
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