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
interface Ihighlight_line {
    id: string,
    number: number,
    setLine: (id: string, number: number) => void;
}
export interface CPU {
    R: string[];
    N: boolean;
    Z: boolean;
    C: boolean;
    V: boolean;
    format: string;
    set_format: (format: string) => void;
    update_cpu: (R: string[], N: boolean, Z: boolean, C: boolean, V: boolean) => void;
}
export interface Memory {
    memory: string[];
    SP: number;
    format: string;
    set_format: (format: string) => void;
    update_memory: (memory: string[], SP: number) => void;
}
export enum DebugStatus {
    RUNNING = 'RUNNING',
    CONTINUE = 'CONTINUE',
    STEP = 'STEP',
    BREAKPOINT = 'BREAKPOINT',
    END = 'END',
}
interface IAssemblyContext {
    cpu: CPU,
    memory: Memory,
    std_out: I_std_out[];    // terminal output
    clear_std_out: () => void;
    push_std_out: (type: string, message: string) => void;
    std_in_active: boolean; // activates input terminal
    set_std_in_active: (update: boolean) => void;
    toolbar_btn: Itoolbar_btn,
    debug_status: DebugStatus,
    set_debug_status: (status: DebugStatus) => void;
    highlight_line: Ihighlight_line,
}

const AssemblyContext = createContext<IAssemblyContext>({
    std_out: [],
    clear_std_out: () => { },
    push_std_out: (type, message) => { },
    std_in_active: false,
    set_std_in_active: (update) => { },
    toolbar_btn: {
        state: {
            continue: false,
            step: false,
            stop: false
        },
        setContinue: (update) => { },
        setStep: (step) => { },
        setStop: (stop) => { },
    },
    debug_status: DebugStatus.END,
    set_debug_status: (status) => { },
    highlight_line: {
        id: '',
        number: 0,
        setLine: (id, number) => { },
    },
    cpu: {
        R: [],
        N: false,
        Z: false,
        C: false,
        V: false,
        format: '',
        set_format: (format) => { },
        update_cpu: (R, N, Z, C, V) => { },
    },
    memory: {
        memory: [],
        SP: 0,
        format: '',
        set_format(format) { },
        update_memory(memory, SP) { },
    }
});

export const AssemblySourceProvider = ({ children }: { children: JSX.Element | JSX.Element[] }) => {
    const [std_out, setSTDOut] = useState<I_std_out[]>([]);
    const [std_in_active, set_std_in_active] = useState(false);
    const [debug_status, set_debug_status] = useState(DebugStatus.CONTINUE);
    const [cpu, setCPU] = useState<CPU>({
        R: new Array(16).fill("0"),
        N: false,
        Z: false,
        C: false,
        V: false,
        format: 'unsigned',
        set_format(format) {
            setCPU(cpu => ({ ...cpu, format }));
        },
        update_cpu(R, N, Z, C, V) {
            setCPU(cpu => ({ ...cpu, R, N, Z, C, V }));
        },
    });
    const [memory, setMemory] = useState<Memory>({
        memory: new Array(1024).fill("0"),
        SP: 0,
        format: 'unsigned',
        set_format(format) {
            setMemory(mem => ({ ...mem, format }));
        },
        update_memory(memory, SP) {
            setMemory(mem => ({ ...mem, memory, SP }));
        },
    });
    const [highlight_line, set_highlite_line] = useState<Ihighlight_line>({
        id: '',
        number: 0,
        setLine: (id, number) => {
            set_highlite_line({ ...highlight_line, id, number })
        },
    });

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
            cpu,
            memory,
            std_out,
            clear_std_out,
            push_std_out,
            std_in_active,
            set_std_in_active,
            toolbar_btn,
            debug_status,
            set_debug_status,
            highlight_line,
        }}>
            {children}
        </AssemblyContext.Provider>
    )
}

export const useAssemblySource = () => {
    return useContext(AssemblyContext);
}