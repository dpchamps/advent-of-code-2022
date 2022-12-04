use "collections"

primitive Fib
    fun fib_recursive(n: USize): USize =>
        match n 
        | 0 => 0
        | 1 => 1
        else 
            Fib.fib_recursive(n-1) + Fib.fib_recursive(n-2)
        end    

    fun fib_memo(n: USize): USize ? => 
        var memo = Array[USize].init(0, n+1)

        for i in Range[USize](0, n+1) do 
            memo(i)? = match i 
                | 0 => 0
                | 1 => 1
                else
                    memo(i-2)?+memo(i-1)?
            end            
        end

        memo(n)?


    fun fib_recursive_memo(n: USize, memo: Map[USize, USize] = Map[USize, USize]): USize ?=> 
        match n
            | 0 => 0
            | 1 => 1
            else
                if memo.contains(n) then
                  return memo(n)?
                end

                let result = fib_recursive_memo(n-1, memo)? + fib_recursive_memo(n-2, memo)?

                memo.update(n, result)

                result
            end
        

actor Main

    new create(env: Env) =>
        let n = try 
            env.args(1)?.usize()?
        else
            0
        end

        try
            let x = Fib.fib_recursive_memo(n)?
            env.out.print("Result: "+x.string())
        else
            env.out.print("Could not compute fib")
        end
        // env.out.print("I'm doing something else!")