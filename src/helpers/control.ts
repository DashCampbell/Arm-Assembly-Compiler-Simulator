import { CPU, DebugStatus, IAssemblyContext, InputStatus } from "@/context/AssemblyContext";
import { ISourceContext } from "@/context/SourceContext";
import { getFileFromName, getFileObject } from "@/stores/files";
import { invoke } from "@tauri-apps/api/tauri";

export const handleCompileRun = (source: ISourceContext, ass_source: IAssemblyContext) => {
    const { directory } = source;
    const {
        clear_std_out,
        push_std_out,
        toolbar_btn,
        set_debug_status
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
            set_debug_status(DebugStatus.RUNNING);

            handleRun(ass_source);
        }
        ).catch(err => {
            err.forEach((mess: string) => {
                push_std_out("error", mess);
            });
            push_std_out("red", "Compiling failed...");
        });
}
export const handleRun = (ass_source: IAssemblyContext, std_input?: number) => {
    const {
        cpu,
        cpu_format,
        memory,
        memory_format,
        push_std_out,
        toolbar_btn,
        input_status,
        set_debug_status
    } = ass_source;

    invoke<[string, InputStatus, DebugStatus]>('run', { std_input })
        .then(([std_out, new_input_status, new_debug_status]) => {
            push_std_out("text", std_out);
            input_status.current = new_input_status;
            if (new_debug_status === DebugStatus.END) {
                push_std_out("run", "Finished Running");
                set_debug_status(DebugStatus.END);
                toolbar_btn.setInactiveMode();
            } else
                toolbar_btn.setRunningMode();
        })
        .catch(err => {
            push_std_out("red", "Runtime Error:");
            push_std_out("error", err);
            push_std_out("run", "Finished Running");
            set_debug_status(DebugStatus.END);
            toolbar_btn.setInactiveMode();
        })
        .finally(() => {
            // Update Terminal, CPU, and Memory data
            invoke<CPU>('display_cpu', { num_format: cpu_format.current }).then(newCPU => {
                cpu.update_cpu(newCPU.R, newCPU.N, newCPU.Z, newCPU.C, newCPU.V);
            });
            invoke<[string[], number]>('display_memory', { num_format: memory_format.current }).then(([ram, sp]) => {
                memory.update_memory(ram.reverse(), sp);
            });
        });
};

// executes one assembly instruction.
export const debug_step = async (source: ISourceContext, ass_source: IAssemblyContext, std_input?: number): Promise<boolean> => {
    const { setSelect, addOpenedFile } = source;
    const {
        cpu,
        cpu_format,
        memory,
        memory_format,
        push_std_out,
        toolbar_btn,
        set_debug_status,
        highlight_line,
        input_status
    } = ass_source;
    let stop = false;
    await invoke<[string, number, DebugStatus, InputStatus, string?,]>('debug_run', { std_input }).then(([file_name, line_number, new_debug_status, new_input_status, std_output]) => {
        if (std_output)
            push_std_out("text", std_output);

        let file = getFileFromName(file_name);
        input_status.current = new_input_status;

        if (file) {
            // set the line to highlight
            highlight_line.setLine(file.id, line_number);
            // open file if it is not already opened
            addOpenedFile(file.id);
            setSelect(file.id);
        }
        if (new_debug_status == DebugStatus.END || new_debug_status == DebugStatus.BREAKPOINT) {
            set_debug_status(new_debug_status);
            stop = true;
            if (new_debug_status == DebugStatus.END) {
                highlight_line.setLine('', 0);  // unhighlight line if program completed
                toolbar_btn.setInactiveMode();
                push_std_out("run", "Finished Debugging");
            } else if (new_debug_status == DebugStatus.BREAKPOINT) {
                toolbar_btn.setBreakpointMode();
            }
        }
        if (new_input_status !== InputStatus.None) {
            toolbar_btn.setRunningMode();
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
        await invoke<CPU>('display_cpu', { num_format: cpu_format.current }).then(res => {
            cpu.update_cpu(res.R, res.N, res.Z, res.C, res.V);
        });
        await invoke<[string[], number]>('display_memory', { num_format: memory_format.current }).then(([ram, sp]) => {
            memory.update_memory(ram.reverse(), sp);
        });
    });
    return Promise.resolve(stop);
}
// continues executing assembly instructions until the last instruction or a breakpoint is reached.
export const debug_continue = async (source: ISourceContext, ass_source: IAssemblyContext, std_input?: number) => {
    let stop = false;
    const { input_status } = ass_source;
    while (!stop && input_status.current === InputStatus.None) {
        stop = await debug_step(source, ass_source, std_input);
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