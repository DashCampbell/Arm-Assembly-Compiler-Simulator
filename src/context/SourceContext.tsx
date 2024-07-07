"use client"

import { createContext, useContext, useState, useCallback } from "react"

interface OpenedFile {
    id: string;
    bSave: boolean; // True - Display save icon, False - File is saved and not changed
    breakpoints: number[];
}
interface ISourceContext {
    selected: string;   // file id
    setSelect: (id: string) => void;
    opened: OpenedFile[];   // list of opened file id's
    addOpenedFile: (id: string) => void;
    delOpenedFile: (id: string) => void;
    directory: string;  // Parent folder
    setDirectory: (dir: string) => void;
    setSaveStateOpenedFile: (id: string, state: boolean) => void;
    updateBreakpoints: (id: string, breakpoints: number[]) => void;
}

const SourceContext = createContext<ISourceContext>({
    selected: '',
    setSelect: (id) => { },
    /**List of opened files */
    opened: [],
    addOpenedFile: (id) => { },
    delOpenedFile: (id) => { },
    directory: '',
    setDirectory: (dir) => { },
    setSaveStateOpenedFile: (id, state) => { },
    updateBreakpoints: (id, breakpoints) => { },
});

export const SourceProvider = ({ children }: { children: JSX.Element | JSX.Element[] }) => {
    const [selected, setSelected] = useState('');   // file id
    const [opened, updateOpenedFiles] = useState<OpenedFile[]>([]); // list of opened file id's
    const [directory, setDirectory] = useState(''); // current directory

    const setSelect = (id: string) => {
        setSelected(id);
    }
    const addOpenedFile = useCallback((id: string) => {
        updateOpenedFiles(prevOpen => {
            if (prevOpen.some(file => file.id === id))
                return prevOpen;    // do nothing if file already opened
            return [...prevOpen, { id, bSave: false, breakpoints: [] }]
        });
    }, [opened]);
    // close opened file
    const delOpenedFile = useCallback((id: string) => {
        updateOpenedFiles(prevOpen => {
            prevOpen = prevOpen.filter(opened => opened.id !== id);
            if (prevOpen.length > 0 && id == selected)
                setSelected(prevOpen[prevOpen.length - 1].id);
            return prevOpen;
        }
        );
    }, [opened]);
    // set save state for opened files
    const setSaveStateOpenedFile = useCallback((id: string, state: boolean) => {
        updateOpenedFiles(prevOpen => {
            let newOpen = [...prevOpen];    // make copy of old state
            if (newOpen.some(o => o.id === id))
                newOpen.find(file => file.id === id)!.bSave = state;
            return newOpen;
        });
    }, [opened]);
    const updateBreakpoints = useCallback((id: string, breakpoints: number[]) => {
        updateOpenedFiles(prevOpen => {
            let newOpen = [...prevOpen];    // make copy of old state
            newOpen.find(file => file.id === id)!.breakpoints = breakpoints;
            return newOpen;
        });
    }, [opened]);

    return (
        <SourceContext.Provider value={{
            selected, setSelect,
            opened,
            addOpenedFile, delOpenedFile,
            directory, setDirectory,
            setSaveStateOpenedFile,
            updateBreakpoints
        }}>
            {children}
        </SourceContext.Provider >
    );
}

export const useSource = (): ISourceContext => {
    return useContext(SourceContext);
}