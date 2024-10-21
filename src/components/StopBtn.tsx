import { DebugStatus, InputStatus, useAssemblySource } from "@/context/AssemblyContext";
import { Icon } from "@iconify/react/dist/iconify.js"
import { invoke } from "@tauri-apps/api/tauri";
import { memo, useMemo } from "react";

function StopBtn({ active }: { active: boolean }) {
    const stop_color = active ? "text-red-500" : "text-slate-400";
    // use memo to prevent unnecessary rerenders, will cause a warning in the console logs
    // const { input_status, highlight_line, toolbar_btn, push_std_out, set_debug_status } = useMemo(() => useAssemblySource(), []);
    const { input_status, highlight_line, toolbar_btn, push_std_out, set_debug_status } = useAssemblySource();
    const handleStop = () => {
        // Stop program from running.
        if (active) {
            invoke('kill_process');
            input_status.current = InputStatus.None;
            highlight_line.setLine('', 0);  // unhighlight line if program completed
            toolbar_btn.setInactiveMode();
            set_debug_status(DebugStatus.END);
            push_std_out("run", "Terminated Program");
        }
    }
    return <span title="stop program"><i onClick={handleStop} className={stop_color + " hover:bg-gray-600 cursor-pointer p-1"}><Icon icon="solar:stop-bold" /></i></span>
}

// Memoize Stop Button component, so that it is not rendered continuously while debugging which makes the onClick event not work.
export default memo(StopBtn);