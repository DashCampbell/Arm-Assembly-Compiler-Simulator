import { useAssemblySource } from "@/context/AssemblyContext";

export default function Terminal() {
    const { std_out } = useAssemblySource();

    return (
        <div id="terminal" className="p-2 bg-zinc-700">
            {/* Standard Output */}
            <div id="terminal-output" className="p-4 bg-primary text-sm text-gray-300 overflow-scroll">
                {std_out.map((out, i) => {
                    switch (out.type) {
                        case "compile":
                            return <p key={i} className=" text-green-500 font-bold">{out.message}</p>
                        case "run":
                            return <p key={i} className="text-green-500 font-bold">{out.message}</p>
                        case "error":
                            return <p key={i}><span className=" text-red-500 font-bold">Error: </span>{out.message}</p>
                        case "red":
                            return <p key={i} className="text-red-500 font-bold">{out.message}</p>
                        default:
                            return <p key={i}>{out.message}</p>
                    }
                })}
            </div>
            {/* Standard Input */}
            <div id="terminal-input" className="py-1 px-6 bg-zinc-600">
                {/* <form action=""> */}
                <span className="text-gray bg-white inline-block pl-2 pr-3 py-1">{">>"}</span>
                <input type="text" placeholder="user input...." className="py-1 w-2/5 focus:outline-none" />
                {/* <input type="submit" hidden/> */}
                {/* </form> */}
            </div>
        </div>
    )
}