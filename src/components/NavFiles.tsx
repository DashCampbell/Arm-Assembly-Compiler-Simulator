"use client"

import { IFile } from "../types/file"
import { MouseEvent } from "react"
import { useSource } from "../context/SourceContext"
import FileIcon from "./FileIcon"
import NavFolderItem from "./NavFolderItem"

interface Props {
    files: IFile[],
    visible: boolean
}

export default function NavFiles({ files, visible }: Props) {
    const { setSelect, selected, addOpenedFile } = useSource();
    /**  Displays file when user clicks on file in sidebar*/
    const onShow = async (ev: React.MouseEvent<Element>, file: IFile) => {
        ev.stopPropagation();
        if (file.kind === 'file') {
            setSelect(file.id);
            addOpenedFile(file.id);
        }
    }

    return (
        <div className={`source-codes ${visible ? '' : 'hidden'}`}>
            {files.map(file => {
                const isSelected = file.id === selected;

                if (file.kind === 'directory') {
                    return <NavFolderItem active={isSelected} key={file.id} file={file} />
                }
                return (
                    <div onClick={(ev) => onShow(ev, file)} key={file.id} className={`source-item ${isSelected ? 'source-item-active' : ''} flex items-center gap-2 px-2 py-0.5 text-gray-500 hover:text-gray-400 cursor-pointer`}>
                        <FileIcon name={file.name} />
                        <span>{file.name}</span>
                    </div>
                );
            })}
        </div >
    );
}