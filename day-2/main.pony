use "files"
use "itertools"
use "collections"

primitive Input
    fun file_to_lines(path: String, auth: AmbientAuth): (Iter[String] ref | None val) => 
        let file_path = FilePath(FileAuth(auth), path)

        match OpenFile(file_path)
        | let file: File =>
            Iter[String](file.lines())
        else
            None   
        end         

primitive Rock
primitive Paper 
primitive Scissors

type GameMove is (Rock | Paper | Scissors)

class GameMatch
    let opponent_move: GameMove
    let my_move: GameMove

    new create(opponent_move': GameMove, my_move': GameMove) => 
        opponent_move = opponent_move'
        my_move = my_move' 

    fun compute_score(): U32 =>
        GameMoveResult.score_from_move(my_move) + GameMoveResult.score_from_moves(opponent_move, my_move)

primitive GameMoveResult

    fun score_from_move(move: GameMove): U32 =>
        match move 
        | Rock => 1
        | Paper => 2
        | Scissors => 3
        end
    
    fun score_from_moves(opponent_move: GameMove, my_move: GameMove): U32 => 
        let win_score: U32 = 6
        let draw_score: U32 = 3
        let lose_score: U32 = 0

        if opponent_move is my_move then return draw_score end
        
        match opponent_move
        | Rock => if my_move is Paper then win_score else lose_score end
        | Paper => if my_move is Scissors then win_score else lose_score end 
        | Scissors => if my_move is Rock then win_score else lose_score end
        end

    fun code_to_move(input: String): GameMove ? =>  
        match input
        | "A" => Rock
        | "B" => Paper
        | "C" => Scissors
        else 
            error
        end

    fun get_lose(move: GameMove): GameMove =>
        match move
        | Rock => Scissors
        | Paper => Rock
        | Scissors => Paper
        end
    
    fun get_win(move: GameMove): GameMove =>
        match move
        | Rock => Paper
        | Paper => Scissors
        | Scissors => Rock
        end
    
    fun derive_move(opponent: GameMove, input: String): GameMove ? =>
        match input
        | "X" => GameMoveResult.get_lose(opponent)
        | "Y" => opponent
        | "Z" => GameMoveResult.get_win(opponent)
        else
            error
        end

actor Main
    new create(env: Env) =>
        match Input.file_to_lines("./input.txt", env.root)
        | let lines: Iter[String] =>
            
            let result = lines.map[(GameMatch | None)]({
                (input: String): (GameMatch| None) => 
                    try
                        let values = input.split(" ")
                        let opponent_move = GameMoveResult.code_to_move(values(0)?)?
                        let my_move = GameMoveResult.derive_move(opponent_move, values(1)?)?

                        GameMatch(opponent_move, my_move)
                    else
                        None
                    end
            })
            .map[U32]({(move: (GameMatch|None)): U32 => match move
                | let move': GameMatch => 
                    let score = move'.compute_score()
                    score
                | None => 
                    env.out.print("Something bad happened")
                    99999999
                end
            })
            .fold[U32](0, {(sum, el) => sum+el})

            env.out.print("Result "+result.string())

            
        else
            env.out.print("Failed to read file")    
        end        