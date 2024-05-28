import { nanoid } from "nanoid";
import { useEffect, useMemo, useRef } from "react";
import { getFileObject } from "../stores/files";
import { readFile, writeFile } from "../helpers/filesys";

// these packages will be used for codemirror
import {basicSetup, EditorView } from "codemirror";
import {keymap} from "@codemirror/view"
import {Compartment} from "@codemirror/state"
import {indentWithTab} from "@codemirror/commands"

// hightlight js, markdown, html, css, json, ...
import { javascript } from "@codemirror/lang-javascript";
import { markdown } from "@codemirror/lang-markdown";
import { html } from "@codemirror/lang-html";
import { css } from "@codemirror/lang-css";
import { json } from "@codemirror/lang-json";
import { rust } from "@codemirror/lang-rust";
import {cpp} from "@codemirror/lang-cpp"
// codemirror theme in dark
import { materialDark } from "cm6-theme-material-dark";
import { useSource } from "@/context/SourceContext";

interface Props {
    id: string;
    active: boolean;
}

export default function CodeEditor({ id, active}: Props) {
    const isRendered = useRef(0);
    const editorId = useMemo(() => nanoid(), []);
    const visible = active ? '' : 'hidden';
    const editorRef = useRef<EditorView | null>(null);
    const {setSaveStateOpenedFile} = useSource();

    // get file metadata by id from /stores/file.ts
    const updateEditorContent = async (id: string) => {
        const file = getFileObject(id);

        readFile(file.path).then(content =>{
            fillContentInEditor(content);
        }).catch(err =>{
            fillContentInEditor("Failed to load " + err);
        });
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
                    // keymap.of([indentWithTab]),
                    javascript(), 
                    markdown(), 
                    html(), 
                    css(), 
                    json(), 
                    rust(), 
                    // cpp(),
                    materialDark,
                ],
                parent: elem,
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
        <main className={`w-full h-full ${visible}`}>
            <div id={editorId} className="root-wrapper h-full" tabIndex={-1} onKeyDown={(ev) =>{
                if (ev.ctrlKey && ev.key === 's') {
                    ev.preventDefault();
                    ev.stopPropagation();
                    // Save file, reset tab icon
                    onSave();
                    setSaveStateOpenedFile(id, false);
                }else if(!ev.ctrlKey && !ev.shiftKey && !ev.altKey && !ev.metaKey){
                    // File is changed
                    // Display "Not saved" icon in tab bar
                    setSaveStateOpenedFile(id, true);
                }
            }}>
            </div>
        </main>
    );
}