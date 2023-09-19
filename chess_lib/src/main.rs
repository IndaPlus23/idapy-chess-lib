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
    squares: [[Option<Piece>; 8]; 8], //Creates a board with 8x8 squares. Every square has an option for a piece that can exist there
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

        let mut initial_board = Board{
            squares: [[None; 8]; 8]
        };

        initial_board.squares[0][0] = Some(Piece::new(Color::White, PieceType::Rook));
        initial_board.squares[0][1] = Some(Piece::new(Color::White, PieceType::Knight));
        initial_board.squares[0][2] = Some(Piece::new(Color::White, PieceType::Bishop));
        initial_board.squares[0][3] = Some(Piece::new(Color::White, PieceType::King));
        initial_board.squares[0][4] = Some(Piece::new(Color::White, PieceType::Queen));
        initial_board.squares[0][5] = Some(Piece::new(Color::White, PieceType::Bishop));
        initial_board.squares[0][6] = Some(Piece::new(Color::White, PieceType::Knight));
        initial_board.squares[0][7] = Some(Piece::new(Color::White, PieceType::Rook));

        for i in 0..8{
            initial_board.squares[1][i] = Some(Piece::new(Color::White, PieceType::Pawn));
        }

        initial_board.squares[7][0] = Some(Piece::new(Color::Black, PieceType::Rook));
        initial_board.squares[7][1] = Some(Piece::new(Color::Black, PieceType::Knight));
        initial_board.squares[7][2] = Some(Piece::new(Color::Black, PieceType::Bishop));
        initial_board.squares[7][3] = Some(Piece::new(Color::Black, PieceType::Queen));
        initial_board.squares[7][4] = Some(Piece::new(Color::Black, PieceType::King));
        initial_board.squares[7][5] = Some(Piece::new(Color::Black, PieceType::Bishop));
        initial_board.squares[7][6] = Some(Piece::new(Color::Black, PieceType::Knight));
        initial_board.squares[7][7] = Some(Piece::new(Color::Black, PieceType::Rook));

        for i in 0..8{
            initial_board.squares[6][i] = Some(Piece::new(Color::Black, PieceType::Pawn));
        }


        let game = Game {
            /* initialise board, set active colour to white, ... */
            player: Player::WhitePlayer,
            state: GameState::InProgress,
            board: initial_board,
            //...
        };

        game

    }


    fn from(&mut self) -> (u32, u32) {

        println!("Which piece do you want to move? (Write on the form; row column)");

        let mut row = 0;
        let mut column = 0;

        loop{

            let input = io::stdin();
 
    
            let square = input 
                .lock()
                .lines()
                .map(|_line| _line.ok().unwrap())
                .collect::<String>();
        

            let rows_and_columns: Vec<u32> = square //Gör om input strängen till en lista.
                .split_whitespace()
                .map(|w|w.parse::<u32>().unwrap())
                .collect();

            row = rows_and_columns[0];
            column = rows_and_columns[1];

            let player_color = match self.player{

                Player::WhitePlayer => Color::White,
                Player::BlackPlayer => Color::Black,
            };

            match self.board.squares[row as usize][column as usize] {
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

        return (row, column);

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

        write!(f, " |:----------------------:|\n")?;
        
        for row in 0..8 {
            write!(f, " | ")?;
            for column in 0..8 {
                if let Some(piece) = self.board.squares[row][column] {
                    // Append the piece's representation to the string
                    let piece_str = match piece.color {
                        Color::White => match piece.piece_type {
                            PieceType::King => "W.K ",
                            PieceType::Queen => "W.Q ",
                            PieceType::Rook => "W.R ",
                            PieceType::Bishop => "W.B ",
                            PieceType::Knight => "W:Kn",
                            PieceType::Pawn => "W.P ",
                        },
                        Color::Black => match piece.piece_type {
                            PieceType::King => "B.K ",
                            PieceType::Queen => "B.Q ",
                            PieceType::Rook => "B.R ",
                            PieceType::Bishop => "B.B ",
                            PieceType::Knight => "B.Kn",
                            PieceType::Pawn => "B.P ",
                        },
                    };
                    write!(f, "{}", piece_str)?;
                } 
                else {
                    // Empty square
                    write!(f, "* ")?;
                }
            }
            write!(f, "|\n")?;
        }
        
        write!(f, " |:----------------------:|")
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