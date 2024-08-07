import { useAssemblySource } from "@/context/AssemblyContext";
import { Icon } from "@iconify/react/dist/iconify.js"
import StopBtn from "@/components/StopBtn"
import { handleDebug, handleContinue, handleRun, handleStep, handleCompileRun } from "@/helpers/control";
import { useSource } from "@/context/SourceContext";

export default function Toolbar() {
    const ass_source = useAssemblySource();
    const source = useSource();
    const { toolbar_btn } = ass_source;

    const activeBtn = (active: boolean): string => {
        return active ? "text-green-400" : "text-slate-400";
    }
    return (
        <div id="toolbar" className="  text-gray-400 py-1 px-2">
            <span>Debug: <i title="debug" onClick={() => handleDebug(source, ass_source)} className={activeBtn(toolbar_btn.state.debug) + " hover:bg-gray-600 cursor-pointer p-1"}><Icon icon="codicon:debug-alt" /></i></span>
            <span title="continue"><i onClick={() => handleContinue(source, ass_source)} className={activeBtn(toolbar_btn.state.continue) + " hover:bg-gray-600 cursor-pointer p-1"}><Icon icon="carbon:continue-filled" /></i></span>
            <span title="step"><i onClick={() => handleStep(source, ass_source)} className={activeBtn(toolbar_btn.state.step) + " hover:bg-gray-600 cursor-pointer p-1"}><Icon icon="clarity:step-forward-solid" /></i></span>
            <span>Run: <i title="run" onClick={() => handleCompileRun(source, ass_source)} className={activeBtn(toolbar_btn.state.run) + " hover:bg-gray-600 cursor-pointer p-1"}><Icon icon="codicon:run-all" /></i></span>
            <StopBtn active={toolbar_btn.state.stop} />
        </div>
    )
}