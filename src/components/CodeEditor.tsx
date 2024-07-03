import { nanoid } from "nanoid";
import { useEffect, useMemo, useRef, useState, useCallback } from "react";
import { getFileObject } from "../stores/files";
import { readFile, writeFile } from "../helpers/filesys";

// these packages will be used for codemirror
import { EditorState, EditorView, Prec, Text, useCodeMirror } from '@uiw/react-codemirror';
import { basicSetup } from "codemirror"
import { keymap } from "@codemirror/view"
import { indentWithTab } from "@codemirror/commands"

// hightlight js, markdown, html, css, json, ...
import { cpp } from "@codemirror/lang-cpp"
// codemirror theme in dark
import { monokai } from "@uiw/codemirror-theme-monokai";
import { useSource } from "@/context/SourceContext";
import { breakpointGutter, getBreakpoints, noFold } from "@/extensions/breakpoint_gutter";
import { hightlight } from "@/extensions/highlight_line";

interface Props {
    id: string;
    selected: boolean;
    content: string;
}

// NOTE: If given an [Object object] error about extensions, make sure all extensions are installed with npm.
// npm i "missing extension"

export default function CodeEditor({ id, selected, content }: Props) {
    const { setSaveStateOpenedFile } = useSource();
    const breakpoints = [1, 2, 5, 20];
    const highlight_line = 29;
    const editor_state = useRef<EditorState | null>(null);
    const editor = useRef<HTMLDivElement | null>(null);
    const extensions = useMemo(() => [
        noFold(),
        basicSetup,
        breakpointGutter(breakpoints ?? []),
        hightlight(highlight_line ?? 0),
    ], [breakpoints, highlight_line]);

    const { view, setContainer } = useCodeMirror({
        container: editor.current,
        height: "100%",
        theme: monokai,
        value: content,
        extensions,
        indentWithTab: true,
        onUpdate(viewUpdate) {
            if (!(viewUpdate.docChanged || viewUpdate.focusChanged || viewUpdate.viewportChanged))
                editor_state.current = viewUpdate.state;
        },
    });
    // get file metadata by id from /stores/file.ts
    // save the content when pressing Ctrl + S
    const onSave = async () => {
        const file = getFileObject(id);
        if (editor_state.current) {
            writeFile(file.path, editor_state.current.doc.toString());
        }
    };
    useEffect(() => {
        // get reference to container
        if (editor.current) {
            setContainer(editor.current)
        }
    }, [editor.current]);
    // scroll to highlighted line, if there is one.
    useEffect(() => {
        if (highlight_line && view) {
            const lines = document.getElementsByClassName('cm-line');
            if (highlight_line > 0 && highlight_line <= lines.length)
                // highlight_line used 1 based index.
                lines[highlight_line - 1].scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
    }, [view, selected, highlight_line]);

    return (
        <main className={(selected ? '' : 'hidden') + " w-full h-full"}>
            <div ref={editor} className="root-wrapper h-full " tabIndex={-1} onKeyDown={(ev) => {
                if (ev.ctrlKey && ev.key === 's') {
                    ev.preventDefault();
                    ev.stopPropagation();
                    // Save file, reset tab icon
                    onSave();
                    setSaveStateOpenedFile(id, false);
                } else if (!ev.ctrlKey && !ev.shiftKey && !ev.altKey && !ev.metaKey) {
                    // File is changed
                    // Display "Not saved" icon in tab bar
                    setSaveStateOpenedFile(id, true);
                }
            }}>
            </div>
        </main>
    );
}