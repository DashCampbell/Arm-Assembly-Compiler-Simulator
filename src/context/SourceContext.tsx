"use client"

import { createContext, useContext, useState, useCallback } from "react"

interface OpenedFile {
    id: string;
    bSave: boolean; // True - Display save icon, False - File is saved and not changed.
}
interface ISourceContext {
    selected: string;   // file id
    setSelect: (id: string) => void;
    opened: OpenedFile[];   // list of opened file id's
    addOpenedFile: (id: string) => void;
    delOpenedFile: (id: string) => void;
    setSaveStateOpenedFile: (id: string, state: boolean) => void;
}

const SourceContext = createContext<ISourceContext>({
    selected: '',
    setSelect: (id) => { },
    /**List of opened files */
    opened: [],
    addOpenedFile: (id) => { },
    delOpenedFile: (id) => { },
    setSaveStateOpenedFile: (id, state) => {},
});

export const SourceProvider = ({ children }: { children: JSX.Element | JSX.Element[] }) => {
    const [selected, setSelected] = useState('');
    const [opened, updateOpenedFiles] = useState<OpenedFile[]>([]);

    const setSelect = (id: string) => {
        setSelected(id);
    }
    const addOpenedFile = useCallback((id: string) => {
        // if (opened.includes(id)) return;    // do nothing if file already opened
        // updateOpenedFiles(prevOpen => ([...prevOpen, id]));
        if (opened.some(o => o.id === id)) return;    // do nothing if file already opened
        updateOpenedFiles(prevOpen => ([...prevOpen, {id, bSave: false}]));
    }, [opened]);
    // close opened file
    const delOpenedFile = useCallback((id: string) => {
        updateOpenedFiles(prevOpen => prevOpen.filter(opened => opened.id !== id));
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

    return (
        <SourceContext.Provider value={{
            selected, setSelect,
            opened,
            addOpenedFile, delOpenedFile,
            setSaveStateOpenedFile,
        }}>
            {children}
        </SourceContext.Provider >
    );
}

export const useSource = () => {
    const { selected, setSelect, opened, addOpenedFile, delOpenedFile , setSaveStateOpenedFile} = useContext(SourceContext)

    return { selected, setSelect, opened, addOpenedFile, delOpenedFile , setSaveStateOpenedFile}
}