import { CPU, DebugStatus, useAssemblySource } from "@/context/AssemblyContext";
import { useSource } from "@/context/SourceContext";
import { getFileFromName, getFileObject } from "@/stores/files";
import { Icon } from "@iconify/react/dist/iconify.js"
import { invoke } from "@tauri-apps/api/tauri";
import { useCallback, useMemo } from "react";
import StopBtn from "@/components/StopBtn"

export default function Toolbar() {
    const { directory, opened } = useSource();
    const {
        cpu,
        memory,
        clear_std_out,
        push_std_out,
        set_std_in_active,
        toolbar_btn,
        set_debug_status,
        highlight_line
    } = useAssemblySource();
    const handleRun = () => {
        // Run Assembly Code
        clear_std_out();

        // Compile Code, update terminal with result
        push_std_out("compile", "Compiling...");

        invoke('compile', { dir_path: directory })
            .then(res => {
                push_std_out("compile", "Compiled Successfully");

                // Run assembly code, activate Stop btn.
                push_std_out("run", "Running...");
                toolbar_btn.setStop(true);

                invoke<string>('run')
                    .then((res: string) => {
                        push_std_out("text", res);
                        push_std_out("run", "Finished Running");
                    })
                    .catch(err => push_std_out("error", err))
                    .finally(() => {
                        // Update Terminal, CPU, and Memory data
                        invoke<CPU>('display_CPU', { num_format: cpu.format }).then(res => {
                            cpu.update_cpu(res.R, res.N, res.Z, res.C, res.V);
                        });
                        invoke<[string[], number]>('display_Memory', { num_format: memory.format }).then(res => {
                            memory.update_memory(res[0].reverse(), res[1]);
                        });
                        // Deactivate Toolbar Buttons
                        toolbar_btn.setStop(false);
                    });
            }
            ).catch(err => {
                err.forEach((mess: string) => {
                    push_std_out("error", mess);
                });
                push_std_out("red", "Compiling failed...");
            });
    };
    const debug_continue = useCallback(async () => {
        let stop = false;
        while (!stop) {
            await invoke<[string, number, string, DebugStatus]>('debug_run').then(([file_name, line_number, std_output, status]) => {
                push_std_out("text", std_output);

                if (status == DebugStatus.END || status == DebugStatus.BREAKPOINT) {
                    set_debug_status(status);
                    stop = true;
                }
                // set the line to highlight
                let file = getFileFromName(file_name);
                highlight_line.setLine(file?.id ?? '', line_number);
            }).catch(err => {
                push_std_out("error", err);
                set_debug_status(DebugStatus.END);
                stop = true;
            }).finally(async () => {
                // Update Terminal, CPU, and Memory data
                // wait until frontend updates, before running next assembly instruction.
                await invoke<CPU>('display_CPU', { num_format: cpu.format }).then(res => {
                    cpu.update_cpu(res.R, res.N, res.Z, res.C, res.V);
                });
                await invoke<[string[], number]>('display_Memory', { num_format: memory.format }).then(([ram, sp]) => {
                    memory.update_memory(ram.reverse(), sp);
                });
            });
        }
    }, []);
    const handleDebug = () => {
        // get breakpoints
        const breakpoint_map: { [name: string]: number[] } = {};
        opened.forEach(({ id, breakpoints }) => {
            const name = getFileObject(id).name;
            breakpoint_map[name as string] = breakpoints;
        });
        clear_std_out();

        // Compile Code, update terminal with result
        push_std_out("compile", "Compiling...");
        invoke('compile', { dir_path: directory, breakpoint_map })
            .then(_ => {
                // Run assembly code, activate Stop btn.
                push_std_out("compile", "Compiled Successfully");
                push_std_out("run", "Debugging...");
                toolbar_btn.setStop(true);
                set_debug_status(DebugStatus.CONTINUE);

                debug_continue().finally(() => {
                    push_std_out("run", "Finished Debugging");
                    // Deactivate Toolbar Buttons
                    toolbar_btn.setStop(false);
                });
            }).catch(err => {
                err.forEach((mess: string) => {
                    push_std_out("error", mess);
                });
                push_std_out("red", "Compiling failed...");
            });
    }
    return (
        <div id="toolbar" className="  text-gray-400 py-1 px-2">
            <span>Debug: <i title="debug" onClick={handleDebug} className=" text-green-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="codicon:debug-alt" /></i></span>
            <span title="continue"><i className="  text-slate-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="carbon:continue-filled" /></i></span>
            <span title="step"><i className=" text-slate-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="clarity:step-forward-solid" /></i></span>
            <span>Run: <i title="run" onClick={handleRun} className=" text-green-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="codicon:run-all" /></i></span>
            <StopBtn active={toolbar_btn.state.stop} />
        </div>
    )
}