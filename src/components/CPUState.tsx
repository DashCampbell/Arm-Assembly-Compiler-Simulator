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
    const { update_cpu, setUpdateCPU } = useAssemblySource();
    const format = useRef<HTMLSelectElement | null>(null);
    const [cpu, setCPU] = useState<CPU>({ R: new Array(16).fill("0"), N: false, Z: false, C: false, V: false });

    const aspr_active = (active: boolean) => (
        active ? "font-bold text-gray-700" : "text-zinc-300"
    );
    useEffect(() => {
        if (update_cpu) {
            if (format.current) {
                invoke<CPU>('display_CPU', { num_format: format.current.value }).then(res => {
                    setCPU(res)
                });
            }
            setUpdateCPU(false);
        }
    }, [update_cpu]);
    return (
        <div id="CPU" className="text-white px-2 py-3 overflow-scroll">
            <h2>Register Values (32-bit)</h2>
            <select onChange={() => setUpdateCPU(true)} defaultValue={"unsigned"} ref={format} title="Display Format of register values." className="block text-zinc-800 my-2 mx-auto p-1 rounded-sm">
                <option value="unsigned">Unsigned Integer</option>
                <option value="signed">Signed Integer</option>
                <option value="binary">Binary</option>
                <option value="hexadecimal">Hexadecimal</option>
            </select>
            <div>
                {[...Array(13)].map((v, i) => (
                    <><span>R{i}: </span><span>{cpu.R[i]}</span></>
                ))}
                <span>SP: </span><span>{cpu.R[13]}</span>
                <span>LR: </span><span>{cpu.R[14]}</span>
                <span>PC: </span><span>{cpu.R[15]}</span>
                <span>APSR</span>
                <span>
                    <span className={aspr_active(true)}>N</span>
                    <span className={aspr_active(cpu.Z)}>Z</span>
                    <span className={aspr_active(cpu.C)}>C</span>
                    <span className={aspr_active(cpu.V)}>V</span>
                </span>
            </div>
        </div >
    )
}