
export default function CPUState() {
    return (
        <div id="CPU" className="text-white px-2 py-3 overflow-scroll">
            <h2>Register Values</h2>
            <select id="" title="Display Format of register values." className="block text-zinc-800 my-2 mx-auto p-1 rounded-sm">
                    <option value="unsigned">Unsigned Integer</option>
                    <option value="signed">Signed Integer</option>
                    <option value="binary">Binary</option>
                    <option value="hexadecimal">Hexadecimal</option>
            </select>
            <div id="register-values">
            {[...Array(13)].map((v, i) => (
                <div key={i}>
                    <span>R{i}: </span><span>0xffff aaaa</span>
                </div>
            ))}
            <div>
                <span>SP: </span><span>0xffff aaaa</span>
            </div>
            <div>
                <span>LR: </span><span>0xffff aaaa</span>
            </div>
            <div>
                <span>PC: </span><span>0xffff aaaa</span>
            </div>
            <div><span>APSR</span><span><span>N</span><span>Z</span><span>C</span><span>V</span></span></div>
            </div>
        </div>
    )
}