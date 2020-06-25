mod board;

enum GameState {
    Start,
    InPlay,
    InCheck,
    End
}

struct ChessEngine {
    state: GameState,
    value: u128,

}


mod moves;

use moves::util as uu;

fn main() {
    println!("Rook:\n");
    for (i, (a,b,c,d)) in moves::rook_moves().iter().enumerate() {
       // println!("Position: {} - {},{},{},{}",i,a,b,c,d);
    }

    println!("Bishop:\n");
    for (i, (a,b,c,d)) in moves::bishop_moves().iter().enumerate() {
        // println!("Position: {} - {},{},{},{} | {}",i,a,b,c,d, a | b | c | d);
    }

    println!("Knight:\n");
    for (i, a) in moves::knight_moves().iter().enumerate() {
        println!("Position: {} - {}",i, a);
    }

}
