import { useAssemblySource } from "@/context/AssemblyContext";
import { Icon } from "@iconify/react/dist/iconify.js"

export default function Toolbar(){
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

        // Reset Terminal, CPU, and Memory data
            // TODO: Invoke tauri command to reset states
            
        // Run assembly code, activate Stop btn.
        push_std_out("run", "Running...");
            // TODO: Invoke tauri command to run code.

        // Update Terminal, CPU, and Memory data
        setUpdateCPU(true);
        setUpdateMemory(true);
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