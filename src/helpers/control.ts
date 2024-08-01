import { CPU, DebugStatus, IAssemblyContext } from "@/context/AssemblyContext";
import { ISourceContext } from "@/context/SourceContext";
import { getFileFromName, getFileObject } from "@/stores/files";
import { invoke } from "@tauri-apps/api/tauri";

export const handleRun = (source: ISourceContext, ass_source: IAssemblyContext) => {
    const { directory } = source;
    const {
        cpu,
        cpu_format,
        memory,
        memory_format,
        clear_std_out,
        push_std_out,
        toolbar_btn,
    } = ass_source;

    if (!toolbar_btn.state.run)
        return;
    // Run Assembly Code
    clear_std_out();

    // Compile Code, update terminal with result
    push_std_out("compile", "Compiling...");

    invoke('compile', { dir_path: directory })
        .then(_ => {
            push_std_out("compile", "Compiled Successfully");

            // Run assembly code, activate Stop btn.
            push_std_out("run", "Running...");
            toolbar_btn.setRunningMode();

            invoke<string | null>('run')
                .then((res) => {
                    if (res)
                        push_std_out("text", res);
                    push_std_out("run", "Finished Running");
                })
                .catch(err => {
                    push_std_out("red", "Runtime Error:");
                    push_std_out("error", err);
                })
                .finally(() => {
                    // Update Terminal, CPU, and Memory data
                    invoke<CPU>('display_CPU', { num_format: cpu_format.current }).then(newCPU => {
                        cpu.update_cpu(newCPU.R, newCPU.N, newCPU.Z, newCPU.C, newCPU.V);
                    });
                    invoke<[string[], number]>('display_Memory', { num_format: memory_format.current }).then(([ram, sp]) => {
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
const debug_step = async (source: ISourceContext, ass_source: IAssemblyContext): Promise<boolean> => {
    const { setSelect, addOpenedFile } = source;
    const {
        cpu,
        cpu_format,
        memory,
        memory_format,
        push_std_out,
        toolbar_btn,
        set_debug_status,
        highlight_line
    } = ass_source;
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
        await invoke<CPU>('display_CPU', { num_format: cpu_format.current }).then(res => {
            cpu.update_cpu(res.R, res.N, res.Z, res.C, res.V);
        });
        await invoke<[string[], number]>('display_Memory', { num_format: memory_format.current }).then(([ram, sp]) => {
            memory.update_memory(ram.reverse(), sp);
        });
    });
    return Promise.resolve(stop);
}
// continues executing assembly instructions until the last instruction or a breakpoint is reached.
const debug_continue = async (source: ISourceContext, ass_source: IAssemblyContext) => {
    let stop = false;
    while (!stop) {
        stop = await debug_step(source, ass_source);
    }
};
export const handleDebug = (source: ISourceContext, ass_source: IAssemblyContext) => {
    const { directory, opened } = source;
    const {
        clear_std_out,
        push_std_out,
        toolbar_btn,
        set_debug_status,
    } = ass_source;

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

            debug_continue(source, ass_source);
        }).catch(err => {
            err.forEach((mess: string) => {
                push_std_out("error", mess);
            });
            push_std_out("red", "Compiling failed...");
        });
}
export const handleContinue = async (source: ISourceContext, ass_source: IAssemblyContext) => {
    const {
        toolbar_btn,
        set_debug_status,
    } = ass_source;
    if (toolbar_btn.state.continue) {
        set_debug_status(DebugStatus.CONTINUE);
        await debug_continue(source, ass_source);
    }
}
export const handleStep = async (source: ISourceContext, ass_source: IAssemblyContext) => {
    const {
        toolbar_btn,
        set_debug_status,
    } = ass_source;
    if (toolbar_btn.state.step) {
        set_debug_status(DebugStatus.STEP);
        await debug_step(source, ass_source);
    }
}