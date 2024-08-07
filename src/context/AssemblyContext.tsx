"use client"

import { createContext, useContext, useState, useCallback, useRef, useMemo, MutableRefObject, createRef, RefObject } from "react"

interface I_std_out {
    type: string,
    message: string,
}
interface Itoolbar_btn {
    state: {
        run: boolean,
        debug: boolean
        continue: boolean,
        step: boolean,
        stop: boolean
    };
    setRunningMode: () => void;
    setBreakpointMode: () => void;
    setInactiveMode: () => void;
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
    update_cpu: (R: string[], N: boolean, Z: boolean, C: boolean, V: boolean) => void;
}
export interface Memory {
    memory: string[];
    SP: number;
    update_memory: (memory: string[], SP: number) => void;
}
export enum InputStatus {
    GetChar = 'GetChar',
    GetNumber = 'GetNumber',
    None = 'None',
}
export enum DebugStatus {
    RUNNING = 'RUNNING',
    CONTINUE = 'CONTINUE',
    STEP = 'STEP',
    BREAKPOINT = 'BREAKPOINT',
    END = 'END',
}

export interface IAssemblyContext {
    cpu: CPU,
    cpu_format: MutableRefObject<string>,
    memory: Memory,
    memory_format: MutableRefObject<string>,
    std_out: I_std_out[];    // terminal output
    clear_std_out: () => void;
    push_std_out: (type: string, message: string) => void;
    toolbar_btn: Itoolbar_btn,
    debug_status: DebugStatus,
    input_status: MutableRefObject<InputStatus>,
    set_debug_status: (status: DebugStatus) => void;
    highlight_line: Ihighlight_line,
}

const AssemblyContext = createContext<IAssemblyContext>({
    std_out: [],
    clear_std_out: () => { },
    push_std_out: (type, message) => { },
    toolbar_btn: {
        state: {
            run: true,
            debug: true,
            continue: false,
            step: false,
            stop: false
        },
        setRunningMode() { },
        setBreakpointMode() { },
        setInactiveMode() { },
    },
    input_status: createRef() as MutableRefObject<InputStatus>,
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
        update_cpu: (R, N, Z, C, V) => { },
    },
    cpu_format: createRef() as MutableRefObject<string>,
    memory: {
        memory: [],
        SP: 0,
        update_memory(memory, SP) { },
    },
    memory_format: createRef() as MutableRefObject<string>,
});

export const AssemblySourceProvider = ({ children }: { children: JSX.Element | JSX.Element[] }) => {
    const [std_out, setSTDOut] = useState<I_std_out[]>([]);
    const [debug_status, set_debug_status] = useState(DebugStatus.END);
    const input_status = useRef<InputStatus>(InputStatus.None);
    const cpu_format = useRef<string>('unsigned');
    const [cpu, setCPU] = useState<CPU>({
        R: new Array(16).fill("0"),
        N: false,
        Z: false,
        C: false,
        V: false,
        update_cpu(R, N, Z, C, V) {
            setCPU(cpu => ({ ...cpu, R, N, Z, C, V }));
        },
    });
    const memory_format = useRef<string>('unsigned');
    const [memory, setMemory] = useState<Memory>({
        memory: new Array(1024).fill("0"),
        SP: 0,
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
            run: true,
            debug: true,
            continue: false,
            step: false,
            stop: false
        },
        setRunningMode: () => { setToolbarBtn({ ...toolbar_btn, state: { run: false, debug: false, continue: false, step: false, stop: true } }) },
        setBreakpointMode: () => { setToolbarBtn({ ...toolbar_btn, state: { run: false, debug: false, continue: true, step: true, stop: true } }) },
        setInactiveMode: () => { setToolbarBtn({ ...toolbar_btn, state: { run: true, debug: true, continue: false, step: false, stop: false } }) },
    });

    const clear_std_out = useCallback(() => {
        setSTDOut(_ => []);
    }, []);
    const push_std_out = useCallback((type: string, message: string) => {
        setSTDOut(std_out => [...std_out, { type, message }]);
    }, []);

    const assemblyValues = useMemo(() => ({
        cpu,
        cpu_format,
        memory,
        memory_format,
        std_out,
        clear_std_out,
        push_std_out,
        toolbar_btn,
        input_status,
        debug_status,
        set_debug_status,
        highlight_line,
    }), [cpu, memory, toolbar_btn, debug_status]);

    return (
        <AssemblyContext.Provider value={assemblyValues}>
            {children}
        </AssemblyContext.Provider>
    )
}

export const useAssemblySource = () => {
    return useContext(AssemblyContext);
}