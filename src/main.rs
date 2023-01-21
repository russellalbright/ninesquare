use itertools::Itertools;

// static mut pieces: [[i32; 4]; 9] = [
//     [3, 5, 8, 4],
//     [3, 5, 8, 2],
//     [9, 3, 5, 6],
//     [3, 4, 1, 2],
//     [6, 5, 7, 1],
//     [8, 5, 4, 1],
//     [1, 9, 7, 4],
//     [7, 5, 6, 9],
//     [3, 2, 6, 7],
// ];
#[derive(Debug)]
struct GamePiece {
    piece: [i32; 4],
}

impl GamePiece {
    pub fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        Self {
            piece: [a, b, c, d],
        }
    }
    fn rotate(&mut self) {
        self.piece.rotate_right(1);
    }
}
#[derive(Debug)]
pub struct GameBoard {
    pieces: Vec<GamePiece>,
    current_board: Vec<i32>,
    lookup_index: Vec<i32>, //
}

// Implementation block, all `Point` associated functions & methods go in here
impl GameBoard {
    pub fn new() -> Self {
        let mut pieces: Vec<GamePiece> = Vec::new();
        let mut piece: GamePiece = GamePiece::new(1, 8, 6, 3);
        pieces.push(piece);
        piece = GamePiece::new(4, 9, 8, 3);
        pieces.push(piece);
        piece = GamePiece::new(2, 9, 7, 4);
        pieces.push(piece);
        piece = GamePiece::new(2, 6, 9, 3);
        pieces.push(piece);
        piece = GamePiece::new(1, 7, 8, 4);
        pieces.push(piece);
        piece = GamePiece::new(2, 6, 9, 3);
        pieces.push(piece);
        piece = GamePiece::new(2, 7, 9, 3);
        pieces.push(piece);
        piece = GamePiece::new(1, 6, 8, 4);
        pieces.push(piece);
        piece = GamePiece::new(2, 7, 9, 4);
        pieces.push(piece);
        let current_board = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let lookup_index = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        Self {
            pieces,
            current_board,
            lookup_index,
        }
    }

    fn find_solution(&mut self) -> bool {
        let mut _trash = 1;
        let start_board = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        for attempt in start_board.iter().permutations(start_board.len()).unique() {
            // if _trash > 2 {
            //     break;
            // }
            // _trash = _trash + 1;
            //println!("attempt {:?}", attempt);
            self.current_board.clear();
            for my_int_ref in attempt.iter() {
                self.current_board.push(**my_int_ref);
            }

            //println!("self.current_board {:?}", self.current_board);
            self.get_lookup_index();
            if self.test_answer() == true {
                println!("{:?}", self.current_board);
                println!("FOUND IT");
                return true;
            } else {
                //println!("NOT FOUND Yet");
                // return false;
            }
        }
        false
    }
    fn get_lookup_index(&mut self) {
        for _i in 1..9 {
            self.lookup_index[self.current_board[_i as usize] as usize] = _i;
        }
    }
    fn test_answer(&mut self) -> bool {
        //     //check 4 surrounding the middle piece
        // for _i in 0..1 {
        if self.check_1_4() == false {
            return false;
        }
        // println!("{:?}  a ", );

        if self.check_3_4() == false {
            return false;
        }
        // println!("{:?}  b", );

        if self.check_4_5() == false {
            return false;
        }
        // println!("{:?}  c", );

        if self.check_4_7() == false {
            return false;
        }
        // println!("{:?}  d", );

        if self.check_0() == false {
            return false;
        }

        if self.check_2() == false {
            return false;
        }

        if self.check_6() == false {
            return false;
        }
        if self.check_8() == false {
            return false;
        }
        true
    }
    fn check_1_4(&mut self) -> bool {
        for _i in 0..4 {
            if self.pieces[self.lookup_index[1] as usize].piece[2]
                + self.pieces[self.lookup_index[4] as usize].piece[0]
                == 10
            {
                return true;
            }
            self.pieces[self.lookup_index[1] as usize].rotate();
        }
        return false;
    }
    fn check_3_4(&mut self) -> bool {
        for _i in 0..4 {
            if self.pieces[self.lookup_index[3] as usize].piece[1]
                + self.pieces[self.lookup_index[4] as usize].piece[3]
                == 10
            {
                return true;
            }
            self.pieces[self.lookup_index[3] as usize].rotate();
        }
        return false;
    }
    fn check_4_5(&mut self) -> bool {
        for _i in 0..4 {
            if self.pieces[self.lookup_index[4] as usize].piece[1]
                + self.pieces[self.lookup_index[5] as usize].piece[3]
                == 10
            {
                return true;
            }
            self.pieces[self.lookup_index[5] as usize].rotate();
        }
        return false;
    }
    fn check_4_7(&mut self) -> bool {
        for _i in 0..4 {
            if self.pieces[self.lookup_index[4] as usize].piece[2]
                + self.pieces[self.lookup_index[7] as usize].piece[0]
                == 10
            {
                return true;
            }
            self.pieces[self.lookup_index[7] as usize].rotate();
        }
        return false;
    }
    fn check_0(&mut self) -> bool {
        for _i in 0..4 {
            // println!(
            //     "{:?} squares {:?}",
            //     self.lookup_index[0], self.lookup_index[1]
            // );
            // println!(
            //     "{:?} and {:?}",
            //     self.pieces[self.lookup_index[0] as usize].piece[1],
            //     self.pieces[self.lookup_index[1] as usize].piece[3]
            // );

            // println!(
            //     "{:?} squares {:?}",
            //     self.lookup_index[0], self.lookup_index[3]
            // );
            // println!(
            //     "{:?} and {:?}",
            //     self.pieces[self.lookup_index[0] as usize].piece[2],
            //     self.pieces[self.lookup_index[3] as usize].piece[0]
            // );

            if self.pieces[self.lookup_index[0] as usize].piece[1]
                + self.pieces[self.lookup_index[1] as usize].piece[3]
                == 10
                && self.pieces[self.lookup_index[0] as usize].piece[2]
                    + self.pieces[self.lookup_index[3] as usize].piece[0]
                    == 10
            {
                return true;
            }
            self.pieces[self.lookup_index[0] as usize].rotate();
        }
        return false;
    }
    fn check_2(&mut self) -> bool {
        for _i in 0..4 {
            if self.pieces[self.lookup_index[1] as usize].piece[1]
                + self.pieces[self.lookup_index[2] as usize].piece[3]
                == 10
                && self.pieces[self.lookup_index[2] as usize].piece[2]
                    + self.pieces[self.lookup_index[5] as usize].piece[0]
                    == 10
            {
                return true;
            }
            self.pieces[self.lookup_index[2] as usize].rotate();
        }
        return false;
    }
    fn check_6(&mut self) -> bool {
        for _i in 0..4 {
            if self.pieces[self.lookup_index[3] as usize].piece[2]
                + self.pieces[self.lookup_index[6] as usize].piece[0]
                == 10
                && self.pieces[self.lookup_index[6] as usize].piece[1]
                    + self.pieces[self.lookup_index[7] as usize].piece[3]
                    == 10
            {
                return true;
            }
            self.pieces[self.lookup_index[6] as usize].rotate();
        }
        return false;
    }
    fn check_8(&mut self) -> bool {
        for _i in 0..4 {
            if self.pieces[self.lookup_index[5] as usize].piece[2]
                + self.pieces[self.lookup_index[8] as usize].piece[0]
                == 10
                && self.pieces[self.lookup_index[7] as usize].piece[1]
                    + self.pieces[self.lookup_index[8] as usize].piece[3]
                    == 10
            {
                return true;
            }
            self.pieces[self.lookup_index[8] as usize].rotate();
        }
        return false;
    }
}

fn main() {
    let mut mygame = GameBoard::new();
    println!("{:?}", mygame);
    if mygame.find_solution() {
        println!("success")
    } else {
        println!("no answer!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rotate() {
        let mut gp = GamePiece::new(4, 9, 8, 3);
        let gp2 = GamePiece::new(3, 4, 9, 8);
        gp.rotate();
        assert_eq!(gp.piece, gp2.piece);
    }
}
