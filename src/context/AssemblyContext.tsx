"use client"

import { createContext, useContext, useState, useCallback, useRef } from "react"

interface I_std_out {
    type: string,
    message: string,
}
interface IAssemblyContext {
    update_cpu: boolean;
    setUpdateCPU: (update: boolean) => void;
    update_memory: boolean;
    setUpdateMemory: (update: boolean) => void;
    std_out: I_std_out[];    // terminal output
    clear_std_out: () => void;
    push_std_out: (type: string, message: string) => void;
    std_in_active: boolean; // activates input terminal
    set_std_in_active: (update: boolean) => void;
    breakpoints: string[];  // breakpoint locations, format = ?
    setBreakpoints: (breakpoints: string[]) => void;
}

const AssemblyContext = createContext<IAssemblyContext>({
    update_cpu: false,
    setUpdateCPU: (update) => {},
    update_memory: false,
    setUpdateMemory: (update)=>{},
    std_out: [],
    clear_std_out: ()=> {},
    push_std_out: (type, message)=>{},
    std_in_active: false,
    set_std_in_active: (update) => {},
    breakpoints: [],
    setBreakpoints: (breakpoints) => {}
});

export const AssemblySourceProvider = ({children}: { children: JSX.Element | JSX.Element[] }) => {
    const [update_cpu, setUpdateCPU] = useState(false);
    const [update_memory, setUpdateMemory] = useState(false);
    const [std_out, setSTDOut] = useState<I_std_out[]>([]);
    const [std_in_active, set_std_in_active] = useState(false);
    const [breakpoints, setBreakpoints] = useState<string[]>([]);

    const clear_std_out = useCallback(()=>{
        setSTDOut(_ => []);
    }, [std_out]);
    const push_std_out = useCallback((type: string, message: string) => {
        setSTDOut(std_out => [...std_out, {type, message}]);
    }, [std_out]);

    return (
        <AssemblyContext.Provider value = {{
            update_cpu,
            setUpdateCPU,
            update_memory,
            setUpdateMemory,
            std_out,
            clear_std_out,
            push_std_out,
            std_in_active,
            set_std_in_active,
            breakpoints,
            setBreakpoints
        }}>
            {children}
        </AssemblyContext.Provider>
    )
}

export const useAssemblySource = () => {
   return useContext(AssemblyContext);
}