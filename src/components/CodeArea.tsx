import { IFile } from "../types/file"
import { useSource } from "../context/SourceContext"
import { getFileObject } from "../stores/files"
import useHorizontalScroll from "../helpers/useHorizontalScroll"
import PreviewImage from "./PreviewImage"
import CodeEditor from "./CodeEditor"
import Tab from "./Tab"
import Terminal from "./Terminal"

export default function CodeArea() {
    const { opened, selected } = useSource();
    const scrollRef = useHorizontalScroll();

    const isImage = (name: string) => {
        return ['.png', '.gif', '.jpeg', 'jpg', '.bmp'].some(ext => name.lastIndexOf(ext) !== -1);
    };

    return (
        <div id="code-area" className="">
            {/** This area is for tab bar */}
            <div ref={scrollRef} className=" h-9 code-tab-items bg-darken flex items-center border-b border-stone-800 divide-x divide-stone-800">
                {opened.map(({ id, bSave }) => {
                    const file = getFileObject(id) as IFile;
                    const active = selected === id ? 'bg-primary text-slate-400' : 'bg-darken';
                    return (
                        <Tab file={file} active={active} id={id} save={bSave} key={id} />
                    )
                })}
            </div>

            {/** This area is for code content */}

            <div className="code-contents">
                {opened.map(({ id }) => {
                    const file = getFileObject(id) as IFile;
                    if (isImage(file.name)) {
                        return <PreviewImage path={file.path} active={id === selected} />
                    }
                    // key must be id and not i, otherwise tabs close wrong editor.
                    if (id === selected)
                        return <CodeEditor key={id} id={id} />
                })}
            </div>
            <Terminal />
        </div>
    );
}