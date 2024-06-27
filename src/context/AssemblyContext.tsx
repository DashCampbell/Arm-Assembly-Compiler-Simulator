"use client"

import { createContext, useContext, useState, useCallback, useRef } from "react"

interface I_std_out {
    type: string,
    message: string,
}
interface Itoolbar_btn {
    state: {
        continue: boolean,
        step: boolean,
        stop: boolean
    };
    setContinue: (update: boolean) => void;
    setStep: (step: boolean) => void;
    setStop: (stop: boolean) => void;
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
    toolbar_btn: Itoolbar_btn,
}

const AssemblyContext = createContext<IAssemblyContext>({
    update_cpu: false,
    setUpdateCPU: (update) => { },
    update_memory: false,
    setUpdateMemory: (update) => { },
    std_out: [],
    clear_std_out: () => { },
    push_std_out: (type, message) => { },
    std_in_active: false,
    set_std_in_active: (update) => { },
    breakpoints: [],
    setBreakpoints: (breakpoints) => { },
    toolbar_btn: {
        state: {
            continue: false,
            step: false,
            stop: false
        },
        setContinue: (update) => { },
        setStep: (step) => { },
        setStop: (stop) => { },
    }
});

export const AssemblySourceProvider = ({ children }: { children: JSX.Element | JSX.Element[] }) => {
    const [update_cpu, setUpdateCPU] = useState(false);
    const [update_memory, setUpdateMemory] = useState(false);
    const [std_out, setSTDOut] = useState<I_std_out[]>([]);
    const [std_in_active, set_std_in_active] = useState(false);
    const [breakpoints, setBreakpoints] = useState<string[]>([]);

    const [toolbar_btn, setToolbarBtn] = useState<Itoolbar_btn>({
        state: {
            continue: false,
            step: false,
            stop: false
        },
        setContinue: (update) => { setToolbarBtn({ ...toolbar_btn, state: { ...toolbar_btn.state, continue: update } }); },
        setStep: (step) => { setToolbarBtn({ ...toolbar_btn, state: { ...toolbar_btn.state, step } }); },
        setStop: (stop) => { setToolbarBtn({ ...toolbar_btn, state: { ...toolbar_btn.state, stop } }); },
    });

    const clear_std_out = useCallback(() => {
        setSTDOut(_ => []);
    }, [std_out]);
    const push_std_out = useCallback((type: string, message: string) => {
        setSTDOut(std_out => [...std_out, { type, message }]);
    }, [std_out]);

    return (
        <AssemblyContext.Provider value={{
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
            setBreakpoints,
            toolbar_btn
        }}>
            {children}
        </AssemblyContext.Provider>
    )
}

export const useAssemblySource = () => {
    return useContext(AssemblyContext);
}