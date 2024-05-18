import { IFile } from "../types/file"
import { useSource } from "../context/SourceContext"
import { getFileObject } from "../stores/files"
import FileIcon from "./FileIcon"
import useHorizontalScroll from "../helpers/useHorizontalScroll" // will be define later
import PreviewImage from "./PreviewImage"
import CodeEditor from "./CodeEditor" // will be define later
import { Icon } from "@iconify/react"

export default function CodeArea() {
    const { opened, selected, setSelect, delOpenedFile } = useSource();
    const scrollRef = useHorizontalScroll();
    const onSelectItem = (id: string) => {
        setSelect(id);
    };
    const isImage = (name: string) => {
        return ['.png', '.gif', '.jpeg', 'jpg', '.bmp'].some(ext => name.lastIndexOf(ext) !== -1);
    };
    const close = (ev: React.MouseEvent<Element>, id: string) => {
        ev.stopPropagation();
        delOpenedFile(id);
    }
    return (
        <div id="code-area" className="h-full">
            {/** This area is for tab bar */}
            <div ref={scrollRef} className="code-tab-items bg-darken flex items-center border-b border-stone-800 divide-x divide-stone-800 overflow-x-auto">
                {opened.map(item => {
                    const file = getFileObject(item) as IFile;
                    const active = selected === item ? 'bg-primary text-slate-400' : 'bg-darken';
                    return (
                        <div onClick={() => onSelectItem(file.id)} className={`tab-item shrink-0 px-3 py-1.5 text-gray-500 cursor-pointer hover:text-gray-400 flex items-center gap-2 ${active}`} key={item}>
                            <FileIcon name={file.name} size="sm" />
                            <span>{file.name}</span>
                            <i onClick={(ev) => close(ev, item)} className="hover:text-red-400"><Icon icon="ri:file-close-line" /></i>
                        </div>
                    )
                })}
            </div>

            {/** This area is for code content */}

            <div className="code-contents">
                {opened.map(item => {
                    const file = getFileObject(item) as IFile;
                    if (isImage(file.name)) {
                        return <PreviewImage path={file.path} active={item === selected} />
                    }
                    return <CodeEditor key={item} id={item} active={item === selected} />

                })}
            </div> 
        </div>
    );
}