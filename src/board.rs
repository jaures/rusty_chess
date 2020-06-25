pub struct Board {
    // Layout
    //  [0] -> Kings
    //  [1] -> Queens
    //  [2] -> Bishops
    //  [3] -> Knights
    //  [4] -> Rooks
    //  [5] -> Pawns
    //  [6] -> White
    board: [u64;7]
}

impl Board {
    // Initialize the Board with Initial Positions
    pub fn new(initial_board: [u64;7]) -> Board {
        Board {
            board: initial_board
        }
    }

    pub fn display(&self) -> String {
        return self.board.show()
    }
}

trait Show {
    fn show(&self) -> String;
}

impl Show for [u64;7]  {
    fn show(&self) -> String {
        let mut res = "".to_string();
        for i in 0..7 {
            println!("{}. {}+{}",i,res, self[i]);
            res = format!("{}{}",res, self[i]);
        }   
        println!("{}", res);
        return res;
    }
}
