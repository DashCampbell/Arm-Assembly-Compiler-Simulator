import { Icon } from "@iconify/react/dist/iconify.js"
import { invoke } from "@tauri-apps/api/tauri";
import { memo } from "react";

function StopBtn({ active }: { active: boolean }) {
    const stop_color = active ? "text-red-500" : "text-slate-400";
    const handleStop = () => {
        // Stop program from running.
        if (active)
            invoke('kill_process');
    }
    return <span title="stop program"><i onClick={handleStop} className={stop_color + " hover:bg-gray-600 cursor-pointer p-1"}><Icon icon="solar:stop-bold" /></i></span>
}

// Memoize Stop Button component, so that it is not rendered continuously while debugging which makes the onClick event not work.
export default memo(StopBtn);