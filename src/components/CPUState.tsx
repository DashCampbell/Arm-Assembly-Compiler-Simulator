
export default function CPUState() {
    return (
        <div id="CPU" className="text-white px-2 py-3 overflow-scroll">
            <h2>Register Values</h2>
            <div id="register-values">
            {[...Array(16)].map((v, i) => (
                <div key={i}>
                    <span>R{i}: </span><span>0xffff aaaa</span>
                </div>
            ))}
            <div><span>CPSR</span><span><span>N</span><span>Z</span><span>C</span><span>V</span></span></div>
            </div>
        </div>
    )
}