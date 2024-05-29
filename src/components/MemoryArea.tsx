import { useRef } from "react";

export default function MemoryArea() {
    const memory_size = 2**9;
    const location = useRef<any | null>(null);
    const inputElement = useRef<HTMLInputElement | null>(null);
    const errorElement = useRef<HTMLParagraphElement | null>(null);

    // https://stackoverflow.com/questions/57803/how-to-convert-decimal-to-hexadecimal-in-javascript
    const decimalToHex = (d: number, padding: number) => {
        var hex = Number(d).toString(16);
        padding = typeof (padding) === "undefined" || padding === null ? padding = 2 : padding;
    
        while (hex.length < padding) {
            hex = "0" + hex;
        }
        return "0x"+hex;
    }
    const handleInput = (key: string)=> {
        if (key === "Enter")
        {
            if (location.current){
                // reset background color of previous selected memory
                location.current.style.backgroundColor = "#81898e";
            }
            if (inputElement.current){
                const input_value = inputElement.current.value; // index of memory location
                let index = memory_size - 1;
                // convert string to number
                if (input_value.match(/^0b[01]+$/i)){
                    index = parseInt(input_value.slice(2), 2);
                }else if (input_value.match(/^\d+$/i) || input_value.match(/^0x[a-fA-F\d]+$/i)){
                    index = parseInt(input_value);
                }else {
                    // handle invalid number
                    if (errorElement.current)
                        errorElement.current.innerText = "Must enter a valid number. eg. 10, 0b1010, 0xa, etc.";
                    return;
                }
                index = memory_size - 1 - index;
                // memory bound checking
                if (index < 0 || index >= memory_size){
                    // handles out of bounds error
                    if (errorElement.current)
                        errorElement.current.innerText = "Memory location is out of bounds.";
                }else{
                    location.current = document.getElementsByClassName("memory-byte")[index];
                    location.current.scrollIntoView({ behavior: "smooth", block: "center", inline: "nearest" });
                    location.current.style.backgroundColor = "cornflowerblue";
                    if (errorElement.current)
                        errorElement.current.innerText = "";
                }
            }
        }
    };
    return (
        <div id="memory-area" className="bg-darken text-white">
            <div className="px-2 pt-3">
                <span className=" mr-1">Go to: </span>
                <input ref={inputElement} type="text" className="text-black rounded-md p-1" placeholder="Memory Address" onKeyDown={(e) => handleInput(e.key)} />
                <select id="" title="Number System of Bytes" className="text-zinc-800 ml-4 p-1 rounded-sm">
                    <option value="decimal">Decimal</option>
                    <option value="binary">Binary</option>
                    <option value="hexadecimal">Hexadecimal</option>
                </select>
            </div>
            <p className="text-xs text-red-500 px-6 h-4" ref={errorElement}>
                {/* Used to output error messages */}
            </p>
            <div id="memory-grid" className="overflow-scroll mt-2 pb-1 text-sm">
                <div>
                    <span>addr + 3</span>
                    <span>addr + 2</span>
                    <span>addr + 1</span>
                    <span>addr + 0</span>
                    <span></span>
                </div>
                {[...Array(memory_size / 4)].map((e, i) =>(
                <div className="memory-row" key={i}>
                    <span className="memory-byte">0b11110000</span>
                    <span className="memory-byte">0b11110000</span>
                    <span className="memory-byte">0b11110000</span>
                    <span className="memory-byte">0b11110000</span>
                    {/* full descending stack */}
                    <span>{decimalToHex(memory_size - i * 4 - 4, 8)}</span>  
                </div>
                ))}
            </div>
        </div>
    )
}