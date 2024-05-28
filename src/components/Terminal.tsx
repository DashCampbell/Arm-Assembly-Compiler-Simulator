export default function Terminal(){
    return (
        <div id="terminal" className=" h-1/4  p-2 bg-zinc-700">
            {/* Standard Output */}
            <div id="terminal-output" className="p-4 bg-primary text-gray-300 overflow-scroll">
                Lorem ipsum dolor sit amet, consectetur adipisicing elit. 
                Nisi nobis atque enim qui eius quo et, inventore quaerat reiciendis nemo delectus
                iste dolorem natus at ab saepe deserunt. Expedita, tempora?
                Lorem ipsum dolor sit amet, consectetur adipisicing elit. 
                Nisi nobis atque enim qui eius quo et, inventore quaerat reiciendis nemo delectus
                iste dolorem natus at ab saepe deserunt. Expedita, tempora?
                Lorem ipsum dolor sit amet, consectetur adipisicing elit. 
                Nisi nobis atque enim qui eius quo et, inventore quaerat reiciendis nemo delectus
                iste dolorem natus at ab saepe deserunt. Expedita, tempora?
            </div>
            {/* Standard Input */}
            <div id="terminal-input" className="py-1 px-6 bg-zinc-600">
                {/* <form action=""> */}
                    <span className="text-gray bg-white inline-block pl-2 pr-3">{">>"}</span>
                    <input type="text" placeholder="user input...." className="w-2/5 focus:outline-none"/>
                    {/* <input type="submit" hidden/> */}
                {/* </form> */}
            </div>
        </div>
    )
}