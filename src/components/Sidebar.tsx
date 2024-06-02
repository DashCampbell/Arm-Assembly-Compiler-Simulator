"use client"

import { open } from "@tauri-apps/api/dialog";
import { useState } from "react";
import { IFile } from "../types/file";
import { readDirectory } from "../helpers/filesys";
import NavFiles from "./NavFiles";
import CPUState from "./CPUState";
import { useSource } from "@/context/SourceContext";


export default function Sidebar() {
    const [projectName, setProjectName] = useState("");
    const [files, setFiles] = useState<IFile[]>([]);
    const { setDirectory } = useSource();

    const loadDirectory = async () => {
        const selected = await open({ directory: true });
        console.log("Open Explorer")

        if (!selected) return;

        readDirectory(selected + "/").then(([files, directory]: [IFile[], string]) => {
            setProjectName(directory);
            setFiles(files);
            setDirectory(selected as string + "\\");
        });
    }

    return (
        <div id="sidebar" className="bg-darken">
            <div className="sidebar-header flex items-center justify-between p-4 py-2.5">
                <button className="project-explorer" onClick={loadDirectory}>File explorer</button>
                <span className="project-name whitespace-nowrap text-gray-400 text-xs">{projectName}</span>
            </div>
            <div className="code-structure overflow-scroll">
                <NavFiles visible={true} files={files} />
            </div>
            <CPUState />
        </div>
    );
}