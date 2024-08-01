import { useAssemblySource } from "@/context/AssemblyContext";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useRef, useState } from "react"

interface CPU {
    R: string[];
    N: boolean;
    Z: boolean;
    C: boolean;
    V: boolean;
}
export default function CPUState() {
    const { cpu, cpu_format } = useAssemblySource();
    const format = useRef<HTMLSelectElement | null>(null);

    const aspr_active = (active: boolean) => (
        active ? "text-gray-800" : "text-zinc-200"
    );
    const update_cpu = () => {
        if (format?.current?.value ?? '' !== cpu_format.current) {
            cpu_format.current = format?.current?.value ?? '';
            invoke<CPU>('display_cpu', { num_format: cpu_format.current }).then(res => {
                cpu.update_cpu(res.R, res.N, res.Z, res.C, res.V);
            });
        }
    }
    return (
        <div id="CPU" className="text-white px-2 py-3 overflow-scroll text">
            <h2>Register Values (32-bit)</h2>
            <select onChange={update_cpu} defaultValue={"unsigned"} ref={format} title="Display Format of register values." className="block text-zinc-800 my-2 mx-auto p-1 rounded-sm">
                <option value="unsigned">Unsigned Integer</option>
                <option value="signed">Signed Integer</option>
                <option value="binary">Binary</option>
                <option value="hexadecimal">Hexadecimal</option>
            </select>
            <div>
                {[...Array(13)].map((_v, i) => (
                    <><span key={i}>R{i}: </span><span key={i + 13}>{cpu.R[i]}</span></>
                ))}
                <span>SP: </span><span>{cpu.R[13]}</span>
                <span>LR: </span><span>{cpu.R[14]}</span>
                <span>PC: </span><span>{cpu.R[15]}</span>
                <span>APSR</span>
                <span className=" text-base font-bold text-gray-800">
                    <span className={"font-sans mx-0.5 " + aspr_active(cpu.N)}>N</span>
                    <span className={"font-sans mx-0.5 " + aspr_active(cpu.Z)}>Z</span>
                    <span className={"font-sans mx-0.5 " + aspr_active(cpu.C)}>C</span>
                    <span className={"font-sans mx-0.5 " + aspr_active(cpu.V)}>V</span>
                </span>
            </div>
        </div >
    )
}