use core::fmt;

const BOARD_SIZE: usize = 3;

#[derive(Copy, Clone)]
enum Piece {
    X,
    O,
    Empty
}



impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rep = match self {
            Piece::X => "X",
            Piece::O => "O",
            Piece::Empty => "-"
        };
        write!(f, "{}", rep)
    }
}

#[derive(Copy, Clone, Debug)]
enum BoardIndex {
    Zero,
    One,
    Two
}

impl From<&BoardIndex> for usize {
    fn from(val: &BoardIndex) -> usize {
        match val {
            BoardIndex::Zero => 0,
            BoardIndex::One => 1,
            BoardIndex::Two => 2,
        }
    }
}

impl BoardIndex {
    fn as_int(&self) -> usize {
        let rtn: usize = self.into();
        rtn
    }
}


#[derive(Copy, Clone)]
struct Board {
    board: [ [Piece; BOARD_SIZE]; BOARD_SIZE]
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"
 {:?} | {:?} | {:?}
---+---+---
 {:?} | {:?} | {:?}
---+---+---
 {:?} | {:?} | {:?} "#,
               self.board[0][0],
               self.board[0][1],
               self.board[0][2],
               self.board[1][0],
               self.board[1][1],
               self.board[1][2],
               self.board[2][0],
               self.board[2][1],
               self.board[2][2],)
    }
}

fn convert_coordinates_to_indices(coor: &str) -> Result<(BoardIndex, BoardIndex), String> {
    if coor.len() != 2 {
        return Err("Invalid Len".to_string())
    }
    let mut str_chars = coor.chars();

    let col = match str_chars.next() {
        Some(c) => {
            match c {
                'A' | 'a' => BoardIndex::Zero,
                'B' | 'b' => BoardIndex::One,
                'C' | 'c' => BoardIndex::Two,
                _ => return Err(format!("Invalid Column Char: {}", c))
            }
        }
        None => return Err("Non-Char".to_string())
    };

    let row = match str_chars.next() {
        Some(c) => {
            match c {
                '1'  => BoardIndex::Zero,
                '2'  => BoardIndex::One,
                '3'  => BoardIndex::Two,
                _ => return Err(format!("Invalid Row Char: {}", c))
            }
        }
        None => return Err("Non-Char".to_string())
    };

    Ok((col, row))
}


impl Board {
    fn new() -> Board {
        Board{
            board: [[Piece::Empty; BOARD_SIZE];BOARD_SIZE]
        }
    }

    fn add_move(self, piece: Piece, coor: &str) -> Result<Board, String> {
        let mut rtn = self.clone();
        let i = convert_coordinates_to_indices(coor)
            .map_err(|e| format!("Invalid Coordinates: {}", e))?;

        if let Piece::Empty = rtn.at_indices(i) {
            rtn.board[i.0.as_int()][i.1.as_int()] = piece;
            Ok(rtn)
        }
        else{
            Err("Move location is not empty".to_string())
        }
    }

    fn at_indices(&self, indices: (BoardIndex, BoardIndex)) -> Piece {
        self.board[indices.0.as_int()][indices.1.as_int()]
    }

    fn at(&self, coor: &str) -> Result<Piece, String> {
        let i = convert_coordinates_to_indices(coor)
            .map_err(|e| format!("Invalid Coordinates: {}", e))?;
        Ok(self.at_indices(i))
    }
}

#[cfg(test)]
mod tests {

    use crate::board::Board;
    use crate::board::BOARD_SIZE;
    use crate::board::convert_coordinates_to_indices;
    use crate::board::Piece;
    use crate::board::BoardIndex;

    #[test]
    fn noop() {

    }

    #[test]
    fn new_board(){
        let b = Board::new();
//        dbg!(b);
    }

    #[test]
    fn coor_convert() {
        let i = convert_coordinates_to_indices("A1");
        assert!(i.is_ok());
        let i_unwrap = i.unwrap();
        assert_eq!(i_unwrap.0.as_int(), 0);
        assert_eq!(i_unwrap.1.as_int(), 0);

        let i = convert_coordinates_to_indices("B1");
        assert!(i.is_ok());
        let i_unwrap = i.unwrap();
        assert_eq!(i_unwrap.0.as_int(), 1);
        assert_eq!(i_unwrap.1.as_int(), 0);

        let i = convert_coordinates_to_indices("C1");
        assert!(i.is_ok());
        let i_unwrap = i.unwrap();
        assert_eq!(i_unwrap.0.as_int(), 2);
        assert_eq!(i_unwrap.1.as_int(), 0);

        let i = convert_coordinates_to_indices("A2");
        assert!(i.is_ok());
        let i_unwrap = i.unwrap();
        assert_eq!(i_unwrap.0.as_int(), 0);
        assert_eq!(i_unwrap.1.as_int(), 1);

        let i = convert_coordinates_to_indices("B2");
        assert!(i.is_ok());
        let i_unwrap = i.unwrap();
        assert_eq!(i_unwrap.0.as_int(), 1);
        assert_eq!(i_unwrap.1.as_int(), 1);

        let i = convert_coordinates_to_indices("C2");
        assert!(i.is_ok());
        let i_unwrap = i.unwrap();
        assert_eq!(i_unwrap.0.as_int(), 2);
        assert_eq!(i_unwrap.1.as_int(), 1);

        let i = convert_coordinates_to_indices("A3");
        assert!(i.is_ok());
        let i_unwrap = i.unwrap();
        assert_eq!(i_unwrap.0.as_int(), 0);
        assert_eq!(i_unwrap.1.as_int(), 2);

        let i = convert_coordinates_to_indices("B3");
        assert!(i.is_ok());
        let i_unwrap = i.unwrap();
        assert_eq!(i_unwrap.0.as_int(), 1);
        assert_eq!(i_unwrap.1.as_int(), 2);

        let i = convert_coordinates_to_indices("C3");
        assert!(i.is_ok());
        let i_unwrap = i.unwrap();
        assert_eq!(i_unwrap.0.as_int(), 2);
        assert_eq!(i_unwrap.1.as_int(), 2);


        assert!(convert_coordinates_to_indices("").is_err());
        assert!(convert_coordinates_to_indices("C33").is_err());
        assert!(convert_coordinates_to_indices("Z3").is_err());
        assert!(convert_coordinates_to_indices("sd").is_err());
        assert!(convert_coordinates_to_indices("22").is_err());
        assert!(convert_coordinates_to_indices(":;").is_err());
    }

    #[test]
    fn add_move() {
        let b = Board::new();
        let r = b.add_move(Piece::X, "A1");
        assert!(r.is_ok());
        let r = r.unwrap();
        if let Piece::X = r.at("A1").unwrap() {} else { panic!("Not correct piece type") }
    }
}