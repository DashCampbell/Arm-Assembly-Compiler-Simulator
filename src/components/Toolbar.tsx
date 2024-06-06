import { useAssemblySource } from "@/context/AssemblyContext";
import { useSource } from "@/context/SourceContext";
import { Icon } from "@iconify/react/dist/iconify.js"
import { invoke } from "@tauri-apps/api/tauri";

export default function Toolbar() {
    const { directory } = useSource();
    const {
        setUpdateCPU,
        setUpdateMemory,
        clear_std_out,
        push_std_out,
        set_std_in_active,
        breakpoints,
    } = useAssemblySource();
    const handleRun = () => {
        // Run Assembly Code
        clear_std_out();

        // Compile Code, update terminal with result
        push_std_out("compile", "Compiling...");
        // TODO: Invoke tauri command to compile code
        console.log(directory);
        invoke('compile', { dir_path: directory })
            .then(res => {
                push_std_out("compile", "Compiled Successfully");

                // Run assembly code, activate Stop btn.
                push_std_out("run", "Running...");

                invoke<string>('run').then((res: string) => {
                    push_std_out("text", res);
                    push_std_out("run", "Finished Running");

                    // Update Terminal, CPU, and Memory data
                    setUpdateCPU(true);
                    setUpdateMemory(true);
                }).catch(err => {
                    push_std_out("error", err);
                });
            }
            ).catch(err => {
                err.forEach((mess: string) => {
                    push_std_out("error", mess);
                });
                push_std_out("red", "Compiling failed...");
            });
    };

    return (
        <div id="toolbar" className="  text-gray-400 py-1 px-2">
            <span>Debug: <i title="debug" className=" text-green-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="codicon:debug-alt" /></i></span>
            <span title="continue"><i className="  text-slate-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="carbon:continue-filled" /></i></span>
            <span title="step"><i className=" text-slate-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="clarity:step-forward-solid" /></i></span>
            <span title="stop program"><i className=" text-slate-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="solar:stop-bold" /></i></span>
            <span>Run: <i title="run" onClick={handleRun} className=" text-green-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="codicon:run-all" /></i></span>
        </div>
    )
}