import { nanoid } from "nanoid";
import { useEffect, useMemo, useRef, useState, useCallback} from "react";
import { getFileObject } from "../stores/files";
import { readFile, writeFile } from "../helpers/filesys";

// these packages will be used for codemirror
import { useCodeMirror } from '@uiw/react-codemirror';
import {basicSetup} from "codemirror"
import {keymap} from "@codemirror/view"
import {indentWithTab} from "@codemirror/commands"

// hightlight js, markdown, html, css, json, ...
import {cpp} from "@codemirror/lang-cpp"
// codemirror theme in dark
import { monokai } from "@uiw/codemirror-theme-monokai";
import { useSource } from "@/context/SourceContext";

interface Props {
    id: string;
    active: boolean;
}

// NOTE: If given an [Object object] error about extensions, make sure all extensions are installed with npm.
// npm i "missing extension"
const extensions = [basicSetup, cpp(),  keymap.of([indentWithTab])];

export default function CodeEditor({ id, active}: Props) {
    const visible = active ? '' : 'hidden';
    const editor = useRef<HTMLDivElement | null>(null);
    const [content, setContent] = useState("Loading...");
    const {setSaveStateOpenedFile} = useSource();

    const { setContainer } = useCodeMirror({
        container: editor.current,
        height: "100%",
        theme: monokai,
        extensions,
        value: content,
        onChange: (value: string) => {
            setContent(value);
        },
    });
    // get file metadata by id from /stores/file.ts
    const updateEditorContent = async (id: string) => {
        const file = getFileObject(id);

        readFile(file.path).then(value =>{
            // fillContentInEditor(value);
            setContent(value);
        }).catch(err =>{
            // fillContentInEditor("Failed to load " + err);
            setContent("Failed to load " + err);
        });
    };
    // save the content when pressing Ctrl + S
    const onSave = async () => {
        const file = getFileObject(id);
        writeFile(file.path, content);
    };
    useEffect(() => {
        updateEditorContent(id);
    }, []);
    useEffect(()=>{
        if (editor.current) {
            setContainer(editor.current);
        }
    }, [editor.current]);


    return (
        <main className={`w-full h-full ${visible}`}>
            <div ref={editor} className="root-wrapper h-full" tabIndex={-1} onKeyDown={(ev) =>{
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