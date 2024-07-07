import { useEffect, useMemo, useRef, useState } from "react";
import { getFileObject } from "../stores/files";
import { writeFile } from "../helpers/filesys";

// these packages will be used for codemirror
import { EditorState, useCodeMirror } from '@uiw/react-codemirror';
import { basicSetup } from "codemirror"

// codemirror theme in dark
import { monokai } from "@uiw/codemirror-theme-monokai";
import { useSource } from "@/context/SourceContext";
import { breakpointGutter, getBreakpoints, noFold } from "@/extensions/breakpoint_gutter";
import { hightlight } from "@/extensions/highlight_line";
import { DebugStatus, useAssemblySource } from "@/context/AssemblyContext";

interface Props {
    id: string;
    selected: boolean;
    content: string;
    breakpoints: number[];
}

// NOTE: If given an [Object object] error about extensions, make sure all extensions are installed with npm.
// npm i "missing extension"

export default function CodeEditor({ id, selected, content, breakpoints }: Props) {
    const { setSaveStateOpenedFile, updateBreakpoints, opened } = useSource();
    const { highlight_line, debug_status } = useAssemblySource();
    const editor = useRef<HTMLDivElement | null>(null);
    const extensions = useMemo(() => [
        noFold(),
        basicSetup,
        breakpointGutter(breakpoints ?? []),
        hightlight((id === highlight_line.id) ? highlight_line.number : 0),
    ], [breakpoints, highlight_line]);

    const { view, setContainer } = useCodeMirror({
        container: editor.current,
        height: "100%",
        theme: monokai,
        value: content,
        extensions,
        indentWithTab: true,
        editable: debug_status === DebugStatus.RUNNING || debug_status === DebugStatus.END,
    });
    // get file metadata by id from /stores/file.ts
    // save the content when pressing Ctrl + S
    const onSave = async () => {
        const file = getFileObject(id);
        if (view) {
            writeFile(file.path, view.state.doc.toString());
        }
    };
    const keyDown = (e: any) => {
        if (e.ctrlKey && e.key === 's') {
            e.preventDefault();
            e.stopPropagation();
            // Save file, reset tab icon
            onSave();
            setSaveStateOpenedFile(id, false);
        } else if (!e.ctrlKey && !e.shiftKey && !e.altKey && !e.metaKey) {
            // File is changed
            // Display "Not saved" icon in tab bar
            if (!opened.find(file => file.id === id)!.bSave)
                setSaveStateOpenedFile(id, true);
        }
    }
    const mouseUp = (e: any) => {
        if (view)
            updateBreakpoints(id, getBreakpoints(view.state));
    }
    useEffect(() => {
        // get reference to container
        if (editor.current) {
            setContainer(editor.current)
        }
    }, [editor.current]);
    // scroll to highlighted line, if there is one.
    useEffect(() => {
        if ((highlight_line.id === id) && view && editor.current) {
            const lines = editor.current.getElementsByClassName('cm-line');
            if (highlight_line.number > 0 && highlight_line.number <= lines.length) {
                // highlight_line used 1 based index.
                lines[highlight_line.number - 1].scrollIntoView({ behavior: 'smooth', block: 'center' });
            }
        }
    }, [view, selected, highlight_line]);

    return (
        <main className={(selected ? '' : 'hidden') + " w-full h-full"}>
            <div ref={editor} className="root-wrapper h-full " tabIndex={-1} onKeyDown={(ev) => { keyDown(ev) }} onMouseUp={(e) => { mouseUp(e) }}>
            </div>
        </main>
    );
}