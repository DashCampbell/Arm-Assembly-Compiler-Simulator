import { Icon } from "@iconify/react/dist/iconify.js"

export default function Toolbar(){
    return (
        <div id="toolbar" className="  text-gray-400 pt-5 px-2">
            <span>Debug: <i className=" text-green-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="codicon:debug-alt" /></i></span>
            <span>Run: <i className=" text-green-400 hover:bg-gray-600 cursor-pointer p-1"><Icon icon="codicon:run-all" /></i></span>
        </div>
    )
}