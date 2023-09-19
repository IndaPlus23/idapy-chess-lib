use std::fmt;
use std::io;
use std::io::prelude::*;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

enum Player{
    WhitePlayer,
    BlackPlayer,
}

#[derive(PartialEq)]
enum Color{
    White,
    Black,
}

enum PieceType{
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

struct Piece{
    color: Color,
    piece_type: PieceType,
}

impl Piece {

    fn new(color: Color, piece_type: PieceType) -> Piece {
        
        let piece = Piece {
            color,
            piece_type,
        };
        
        piece
    }
}

struct Board {
    squares: Vec<Option<Piece>>,
}

impl Board {
    fn new() -> Self {
        let mut squares = Vec::with_capacity(64); // Create a vector with a capacity of 64 squares

        // Initialize the vector with None values for each square
        for _ in 0..64 {
            squares.push(None);
        }

        squares[0] = Some(Piece::new(Color::White, PieceType::Rook));
        squares[1] = Some(Piece::new(Color::White, PieceType::Knight));
        squares[2] = Some(Piece::new(Color::White, PieceType::Bishop));
        squares[3] = Some(Piece::new(Color::White, PieceType::King));
        squares[4] = Some(Piece::new(Color::White, PieceType::Queen));
        squares[5] = Some(Piece::new(Color::White, PieceType::Bishop));
        squares[6] = Some(Piece::new(Color::White, PieceType::Knight));
        squares[7] = Some(Piece::new(Color::White, PieceType::Rook));

        for i in 8..16{
            squares[i] = Some(Piece::new(Color::White, PieceType::Pawn));
        }

        squares[56] = Some(Piece::new(Color::Black, PieceType::Rook));
        squares[57] = Some(Piece::new(Color::Black, PieceType::Knight));
        squares[58] = Some(Piece::new(Color::Black, PieceType::Bishop));
        squares[59] = Some(Piece::new(Color::Black, PieceType::Queen));
        squares[60] = Some(Piece::new(Color::Black, PieceType::King));
        squares[61] = Some(Piece::new(Color::Black, PieceType::Bishop));
        squares[62] = Some(Piece::new(Color::Black, PieceType::Knight));
        squares[63] = Some(Piece::new(Color::Black, PieceType::Rook));

        for i in 48..56{
            squares[i] = Some(Piece::new(Color::Black, PieceType::Pawn));
        }

        Board { squares }

    }
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 * - Read the Rust documentation, ask questions if you get stuck!
 */

pub struct Game {
    
    player: Player,
    state: GameState,
    board: Board,
    
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {

        let initial_board = Board::new();


        let game = Game {
            /* initialise board, set active colour to white, ... */
            player: Player::WhitePlayer,
            state: GameState::InProgress,
            board: initial_board,
            //...
        };

        game

    }


    fn from(&mut self) -> u32 {

        println!("Which piece do you want to move? (Write the number of the square it is )");

        let mut place = 0;

        loop{

            let input = io::stdin();
 
    
            let mut place1 = input 
                .lock()
                .lines()
                .map(|_line| _line.ok().unwrap())
                .collect::<Vec<String>>();

            place = place1.get(0).unwrap().parse().unwrap();

            match &self.board.squares[place as usize] {

                Some(_Piece) => {

                    let player_color = match self.player{

                        Player::WhitePlayer => Color::White,
                        Player::BlackPlayer => Color::Black,
                    };
        
                    match &self.board.squares[place as usize] {
                        Some(Piece) => {
        
                            if player_color == Piece.color{
                                break;
                            }
                            else{
                                println!("This square contains your opponents piece. Try selecting another square!");
                            }
                        },
                        None => {
                            println!("There is no piece at this square! Try selecting another square!");
                        }
                    }
                }

                None => {
                    println!("Invalid input! Please enter a number between 0 and 63")
                }
            }
        }

        place

    }
    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        None
    }

    /// (Optional but recommended) Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Implement en passant and castling.
    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {
        None
    }
    
}

fn main() {
    let mut game = Game::new();

    println!("{:?}", game); //Initializes a new game with the board set up in the initial position, game state as active and player as white
}







/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut board_string = String::new();

        for row in 0..8 {
            for column in 0..8 {
                let index = row * 8 + column;
                if let Some(piece) = &self.board.squares[index] {
                    // Append the piece's representation to the string
                    let piece_str = match piece.color {
                        Color::White => match piece.piece_type {
                            PieceType::King => "W.K",
                            PieceType::Queen => "W.Q",
                            PieceType::Rook => "W.R",
                            PieceType::Bishop => "W.B",
                            PieceType::Knight => "W.Kn",
                            PieceType::Pawn => "W.P",
                        },
                        Color::Black => match piece.piece_type {
                            PieceType::King => "B.K",
                            PieceType::Queen => "B.Q",
                            PieceType::Rook => "B.R",
                            PieceType::Bishop => "B.B",
                            PieceType::Knight => "B.Kn",
                            PieceType::Pawn => "B.P",
                        },
                    };
                    board_string.push_str(piece_str);
                } else {
                    // Empty square
                    board_string.push(' ');
                }
            }
            board_string.push('\n');
        }

        write!(f, "{}", board_string)
        
    }
        
}




// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}