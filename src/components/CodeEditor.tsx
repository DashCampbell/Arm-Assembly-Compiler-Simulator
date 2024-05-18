import { nanoid } from "nanoid";
import { useEffect, useMemo, useRef } from "react";
import { getFileObject } from "../stores/files";
import { readFile, writeFile } from "../helpers/filesys";

// these packages will be used for codemirror
import { EditorView, basicSetup } from "codemirror";
// hightlight js, markdown, html, css, json, ...
import { javascript } from "@codemirror/lang-javascript";
import { markdown } from "@codemirror/lang-markdown";
import { html } from "@codemirror/lang-html";
import { css } from "@codemirror/lang-css";
import { json } from "@codemirror/lang-json";
import { rust } from "@codemirror/lang-rust";
// codemirror theme in dark
import { materialDark } from "cm6-theme-material-dark";

interface Props {
    id: string;
    active: boolean;
}

export default function CodeEditor({ id, active }: Props) {
    const isRendered = useRef(0);
    const editorId = useMemo(() => nanoid(), []);
    const visible = active ? '' : 'hidden';
    const editorRef = useRef<EditorView | null>(null);

    // get file metadata by id from /stores/file.ts
    const updateEditorContent = async (id: string) => {
        const file = getFileObject(id);
        const content = await readFile(file.path);

        fillContentInEditor(content);
    };

    // fill content into codemirror
    const fillContentInEditor = (content: string) => {
        const elem = document.getElementById(editorId);

        if (elem && isRendered.current === 0) {
            isRendered.current = 1;
            editorRef.current = new EditorView({
                doc: content,
                extensions: [
                    basicSetup,
                    javascript(), markdown(), html(), css(), json(), rust(),
                    materialDark
                ],
                parent: elem
            });
        }
    };
    // save the content when pressing Ctrl + S
    const onSave = async () => {
        if (!editorRef.current) return;

        const content = editorRef.current.state.doc.toString();
        const file = getFileObject(id);

        writeFile(file.path, content);
    };
    useEffect(() => {
        updateEditorContent(id);
    }, [id]);

    return (
        <main className={`w-full ${visible}`} style={{ height: 'calc(100vh - 40px' }}>
            <div id={editorId} className="root-wrapper" tabIndex={-1} onKeyUp={(ev) => {
                if (ev.ctrlKey && ev.key === 's') {
                    ev.preventDefault();
                    ev.stopPropagation();
                    onSave();
                }
            }} onKeyDown={(ev) =>{
                if (ev.key === 'Tab'){
                    ev.preventDefault();
                    ev.stopPropagation();
                } 
            }}></div>
        </main>
    );
}