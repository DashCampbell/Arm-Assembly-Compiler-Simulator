

export default function MemoryArea() {
    return (
        <div id="memory-area" className=" h-screen bg-darken text-white overflow-hidden">
            <div className="px-2 py-3"><span className=" mr-1">Go to: </span><input type="text" className="text-black rounded-md p-1" placeholder="Memory Address"/></div>
            <div id="memory-grid" className="overflow-scroll pb-2 text-sm">
                <div>
                    <span>addr + 3</span>
                    <span>addr + 2</span>
                    <span>addr + 1</span>
                    <span>addr + 0</span>
                    <span></span>
                </div>
                {[...Array(100)].map((e, i) =>(
                <div key={i}>
                    <span>0000 0000</span>
                    <span>0000 0000</span>
                    <span>0000 0000</span>
                    <span>0000 0000</span>
                    <span>0xaaaa 00b{i % 10}</span>
                </div>
                ))}
            </div>
        </div>
    )
}