use "files"
use "itertools"
use "collections"
use "serialise"

primitive Input
    fun file_to_lines(path: String, auth: AmbientAuth): (Iter[String] ref | None val) => 
        let file_path = FilePath(FileAuth(auth), path)

        match OpenFile(file_path)
        | let file: File =>
            Iter[String](file.lines())
        else
            None   
        end 

primitive Utils
    fun maybe_to_u32(input: String): (U32 val | None val) =>
        try input.u32()? else None end
    
    fun into_tuple_folder(acc: Array[Array[U32]], el: (U32 val | None val)): Array[Array[U32]] =>
        match el
        | let calories: U32 =>
            try
                acc(acc.size()-1)?.push(calories)
            end    
            acc
        | None => 
            acc.push(Array[U32](1))
            acc
        end    

    fun sum_array(input: Array[U32]): U32 => 
        Iter[U32](input.values())
            .fold[U32](0, ({(acc, el) => acc+el}))

primitive CalorieCounter
    fun find_elf_with_most_calories(list: Iter[String], count: USize): U32 ? => 
        let pairs = list
            .map[(U32 val | None val)]({(x) => Utils.maybe_to_u32(x)})
            .fold[Array[Array[U32]]](
                Array[Array[U32]].init(Array[U32](1), 1),
                {(acc, el) => Utils.into_tuple_folder(acc, el)}
            ) 
        
        let heap = Iter[Array[U32]](pairs.values())    
            .map[U32]({(input: Array[U32]) => Utils.sum_array(input)})
            .fold[BinaryHeap[U32, MaxHeapPriority[U32]]](
                BinaryHeap[U32, MaxHeapPriority[U32]](pairs.size()),
                {
                    (acc, el) => 
                        acc.push(el)
                        acc
                }
            )

        var result: U32 = 0    

        for i in Range(0, count) do
            result = result + heap.pop()?
        end    
        
        result

actor Main
    new create(env: Env) =>
        match Input.file_to_lines("./input.txt", env.root)
        | let lines: Iter[String] =>
            try
                let result = CalorieCounter.find_elf_with_most_calories(lines, 1)?
                env.out.print("Result: " + result.string())
            else
                env.out.print("Couldn't get the calories")   
            end
        else
            env.out.print("Couldn't get the lines")    
        end







