import { useAssemblySource } from "@/context/AssemblyContext";
import { invoke } from "@tauri-apps/api/tauri";
import { Fragment, useEffect, useRef, useState } from "react";

export default function MemoryArea() {
    const location = useRef<any | null>(null);
    const inputElement = useRef<HTMLInputElement | null>(null);
    const errorElement = useRef<HTMLParagraphElement | null>(null);

    const format = useRef<HTMLSelectElement | null>(null);
    const { memory, memory_format } = useAssemblySource();


    // https://stackoverflow.com/questions/57803/how-to-convert-decimal-to-hexadecimal-in-javascript
    const decimalToHex = (d: number, padding: number) => {
        var hex = Number(d).toString(16);
        padding = typeof (padding) === "undefined" || padding === null ? padding = 2 : padding;

        while (hex.length < padding) {
            hex = "0" + hex;
        }
        return "0x" + hex;
    }
    const handleInput = (key: string) => {
        if (key === "Enter") {
            if (location.current) {
                // reset background color of previous selected memory
                location.current.style.backgroundColor = "";
            }
            if (inputElement.current) {
                const input_value = inputElement.current.value; // index of memory location
                let index = memory.memory.length - 1;
                // convert string to number
                if (input_value.match(/^0b[01]+$/i)) {
                    index = parseInt(input_value.slice(2), 2);
                } else if (input_value.match(/^\d+$/i) || input_value.match(/^0x[a-fA-F\d]+$/i)) {
                    index = parseInt(input_value);
                } else {
                    // handle invalid number
                    if (errorElement.current)
                        errorElement.current.innerText = "Must enter a valid number. eg. 10, 0b1010, 0xa, etc.";
                    return;
                }
                index = memory.memory.length - 1 - index;
                // memory bound checking
                if (index < 0 || index >= memory.memory.length) {
                    // handles out of bounds error
                    if (errorElement.current)
                        errorElement.current.innerText = "Memory location is out of bounds.";
                } else {
                    location.current = document.getElementsByClassName("memory-byte")[index];
                    location.current.scrollIntoView({ behavior: "smooth", block: "center", inline: "nearest" });
                    location.current.style.backgroundColor = "cornflowerblue";
                    if (errorElement.current)
                        errorElement.current.innerText = "";
                }
            }
        }
    };
    const updateMemory = () => {
        if (format?.current?.value ?? '' !== memory_format.current) {
            memory_format.current = format?.current?.value ?? '';
            invoke<[string[], number]>('display_memory', { num_format: memory_format.current }).then(([ram, sp]) => {
                memory.update_memory(ram.reverse(), sp);
            });
        }
    }
    return (
        <div id="memory-area" className="bg-darken text-white">
            <div className="px-2 pt-3">
                <span className=" mr-1">Go to: </span>
                <input ref={inputElement} type="text" className="text-black rounded-md p-1" placeholder="Memory Address" onKeyDown={(e) => handleInput(e.key)} />
                <select ref={format} onChange={updateMemory} defaultValue={"unsigned"} title="Display Format of bytes" className="text-zinc-800 ml-4 p-1 rounded-sm">
                    <option value="unsigned">Unsigned Integer</option>
                    <option value="signed">Signed Integer</option>
                    <option value="binary">Binary</option>
                    <option value="hexadecimal">Hexadecimal</option>
                </select>
            </div>
            <p className="text-xs text-red-500 px-6 h-4" ref={errorElement}>
                {/* Used to output error messages */}
            </p>
            <p className=" px-3 text-center"><span className=" text-purple-500 font-bold">&#9632; </span><span className=" text-sm">Stack Pointer</span></p>
            <div id="memory-grid" className="overflow-scroll mt-2 pb-1 text-sm">
                <span className="sticky top-0">addr + 3</span>
                <span className="sticky top-0">addr + 2</span>
                <span className="sticky top-0">addr + 1</span>
                <span className="sticky top-0">addr + 0</span>
                <span className="sticky top-0">address</span>
                {/* full descending stack */}
                {memory.memory.map((byte, i) => (
                    <Fragment key={i}>
                        <span className={"memory-byte" + ((memory.memory.length - i - 1) == memory.SP ? " bg-purple-500" : " bg-[#81898e]")}>{byte}</span>
                        {
                            (i + 1) % 4 == 0 ?
                                <span>{decimalToHex(memory.memory.length - i - 1, 8)}</span> :
                                null
                        }
                    </Fragment>
                ))}
            </div>
        </div>
    )
}