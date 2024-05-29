import { Icon } from "@iconify/react/dist/iconify.js"

export default function Toolbar(){
    return (
        <div id="toolbar" className="  text-gray-400 py-1 px-2">
            <span>Debug: <i title="debug" className=" text-green-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="codicon:debug-alt" /></i></span>
            <span title="continue"><i className="  text-slate-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="carbon:continue-filled" /></i></span>
            <span title="step"><i className=" text-slate-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="clarity:step-forward-solid" /></i></span>
            <span title="stop program"><i className=" text-slate-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="solar:stop-bold" /></i></span>
            <span>Run: <i title="run" className=" text-green-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="codicon:run-all" /></i></span>
        </div>
    )
}