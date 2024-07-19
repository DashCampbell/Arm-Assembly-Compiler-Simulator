import { CPU, DebugStatus, useAssemblySource } from "@/context/AssemblyContext";
import { useSource } from "@/context/SourceContext";
import { getFileFromName, getFileObject } from "@/stores/files";
import { Icon } from "@iconify/react/dist/iconify.js"
import { invoke } from "@tauri-apps/api/tauri";
import { useCallback, useMemo } from "react";
import StopBtn from "@/components/StopBtn"

export default function Toolbar() {
    const { directory, opened, setSelect, addOpenedFile } = useSource();
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
    const activeBtn = (active: boolean): string => {
        return active ? "text-green-400" : "text-slate-400";
    }
    const handleRun = () => {
        if (!toolbar_btn.state.run)
            return;
        // Run Assembly Code
        clear_std_out();

        // Compile Code, update terminal with result
        push_std_out("compile", "Compiling...");

        invoke('compile', { dir_path: directory })
            .then(std_out => {
                push_std_out("compile", "Compiled Successfully");

                // Run assembly code, activate Stop btn.
                push_std_out("run", "Running...");
                toolbar_btn.setRunningMode();

                invoke<string>('run')
                    .then((res: string) => {
                        push_std_out("text", res);
                        push_std_out("run", "Finished Running");
                    })
                    .catch(err => {
                        push_std_out("red", "Runtime Error:");
                        push_std_out("error", err);
                    })
                    .finally(() => {
                        // Update Terminal, CPU, and Memory data
                        invoke<CPU>('display_CPU', { num_format: cpu.format }).then(newCPU => {
                            cpu.update_cpu(newCPU.R, newCPU.N, newCPU.Z, newCPU.C, newCPU.V);
                        });
                        invoke<[string[], number]>('display_Memory', { num_format: memory.format }).then(([ram, sp]) => {
                            memory.update_memory(ram.reverse(), sp);
                        });
                        // Deactivate Toolbar Buttons
                        toolbar_btn.setInactiveMode();
                    });
            }
            ).catch(err => {
                err.forEach((mess: string) => {
                    push_std_out("error", mess);
                });
                push_std_out("red", "Compiling failed...");
            });
    };
    // executes one assembly instruction.
    const debug_step = useCallback(async (): Promise<boolean> => {
        let stop = false;
        await invoke<[string, number, string, DebugStatus]>('debug_run').then(([file_name, line_number, std_output, status]) => {
            push_std_out("text", std_output);
            let file = getFileFromName(file_name);
            if (file) {
                // set the line to highlight
                highlight_line.setLine(file.id, line_number);
                // open file if it is not already opened
                addOpenedFile(file.id);
                setSelect(file.id);
            }
            if (status == DebugStatus.END || status == DebugStatus.BREAKPOINT) {
                set_debug_status(status);
                stop = true;
                if (status == DebugStatus.END) {
                    highlight_line.setLine('', 0);  // unhighlight line if program completed
                    toolbar_btn.setInactiveMode();
                    push_std_out("run", "Finished Debugging");
                } else if (status == DebugStatus.BREAKPOINT) {
                    toolbar_btn.setBreakpointMode();
                }
            }
        }).catch(err => {
            push_std_out("error", err);
            set_debug_status(DebugStatus.END);
            highlight_line.setLine('', 0);  // unhighlight line if program completed
            toolbar_btn.setInactiveMode();
            push_std_out("run", "Finished Debugging");
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
        return Promise.resolve(stop);
    }, [cpu, memory]);
    // continues executing assembly instructions until the last instruction or a breakpoint is reached.
    const debug_continue = async () => {
        let stop = false;
        while (!stop) {
            stop = await debug_step();
        }
    };
    const handleDebug = () => {
        if (!toolbar_btn.state.debug)
            return;
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
                toolbar_btn.setRunningMode();
                set_debug_status(DebugStatus.CONTINUE);

                debug_continue();
            }).catch(err => {
                err.forEach((mess: string) => {
                    push_std_out("error", mess);
                });
                push_std_out("red", "Compiling failed...");
            });
    }
    const handleContinue = async () => {
        if (toolbar_btn.state.continue) {
            set_debug_status(DebugStatus.CONTINUE);
            await debug_continue();
        }
    }
    const handleStep = async () => {
        if (toolbar_btn.state.step) {
            set_debug_status(DebugStatus.STEP);
            await debug_step();
        }
    }
    return (
        <div id="toolbar" className="  text-gray-400 py-1 px-2">
            <span>Debug: <i title="debug" onClick={handleDebug} className={activeBtn(toolbar_btn.state.debug) + " hover:bg-gray-600 cursor-pointer p-1"}><Icon icon="codicon:debug-alt" /></i></span>
            <span title="continue"><i onClick={handleContinue} className={activeBtn(toolbar_btn.state.continue) + " hover:bg-gray-600 cursor-pointer p-1"}><Icon icon="carbon:continue-filled" /></i></span>
            <span title="step"><i onClick={handleStep} className={activeBtn(toolbar_btn.state.step) + " hover:bg-gray-600 cursor-pointer p-1"}><Icon icon="clarity:step-forward-solid" /></i></span>
            <span>Run: <i title="run" onClick={handleRun} className={activeBtn(toolbar_btn.state.run) + " hover:bg-gray-600 cursor-pointer p-1"}><Icon icon="codicon:run-all" /></i></span>
            <StopBtn active={toolbar_btn.state.stop} />
        </div>
    )
}