import { IFile } from "../types/file"
import { useSource } from "../context/SourceContext"
import { getFileObject } from "../stores/files"
import useHorizontalScroll from "../helpers/useHorizontalScroll" // will be define later
import PreviewImage from "./PreviewImage"
import CodeEditor from "./CodeEditor" // will be define later
import { useState, useRef, useEffect } from "react"
import Tab from "./Tab"
import Terminal from "./Terminal"

export default function CodeArea() {
    const { opened, selected} = useSource();
    const scrollRef = useHorizontalScroll();
   
    const isImage = (name: string) => {
        return ['.png', '.gif', '.jpeg', 'jpg', '.bmp'].some(ext => name.lastIndexOf(ext) !== -1);
    };
    // TODO: Keep track of save states, and update tab icons accordingly.

    return (
        <div id="code-area" className="">
            {/** This area is for tab bar */}
            <div ref={scrollRef} className=" h-9 code-tab-items bg-darken flex items-center border-b border-stone-800 divide-x divide-stone-800">
                {opened.map(({id, bSave}) => {
                    const file = getFileObject(id) as IFile;
                    const active = selected === id ? 'bg-primary text-slate-400' : 'bg-darken';
                    return (
                        <Tab file={file} active={active} id={id} save={bSave} key={id}/>
                    )
                })}
            </div>

            {/** This area is for code content */}

            <div className="code-contents">
                {opened.map(({id}, i) => {
                    const file = getFileObject(id) as IFile;
                    if (isImage(file.name)) {
                        return <PreviewImage path={file.path} active={id === selected} />
                    }
                    // key must be id and not i, otherwise tabs close wrong editor.
                    return <CodeEditor key={id} id={id} active={id === selected} />
            })}
            </div> 
            <Terminal />
        </div>
    );
}