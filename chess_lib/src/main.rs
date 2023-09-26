use std::fmt;
use std::io;
use std::io::prelude::*;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Player{
    WhitePlayer,
    BlackPlayer,
}

#[derive(PartialEq)]
enum Color{
    White,
    Black,
}


#[derive(PartialEq)]
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
        squares[3] = Some(Piece::new(Color::White, PieceType::Queen));
        squares[4] = Some(Piece::new(Color::White, PieceType::King));
        squares[5] = Some(Piece::new(Color::White, PieceType::Bishop));
        squares[6] = Some(Piece::new(Color::White, PieceType::Knight));
        squares[7] = Some(Piece::new(Color::White, PieceType::Rook));

        for i in 8..16{
            squares[i] = Some(Piece::new(Color::White, PieceType::Pawn));
        }

        squares[56] = Some(Piece::new(Color::Black, PieceType::Rook));
        squares[57] = Some(Piece::new(Color::Black, PieceType::Knight));
        squares[58] = Some(Piece::new(Color::Black, PieceType::Bishop));
        squares[59] = Some(Piece::new(Color::Black, PieceType::King));
        squares[60] = Some(Piece::new(Color::Black, PieceType::Queen));
        squares[61] = Some(Piece::new(Color::Black, PieceType::Bishop));
        squares[62] = Some(Piece::new(Color::Black, PieceType::Knight));
        squares[63] = Some(Piece::new(Color::Black, PieceType::Rook));

        for i in 48..56{
            squares[i] = Some(Piece::new(Color::Black, PieceType::Pawn));
        }

        Board { squares }

    }
}


pub struct Game {
    
    player: Player,
    state: GameState,
    board: Board,
    
}


impl Game {
    /// Initialises a new board. Sets the game state to in progress and player to white player.
    pub fn new() -> Game {

        let initial_board = Board::new();


        let game = Game {

            player: Player::WhitePlayer,
            state: GameState::InProgress,
            board: initial_board,
            
        };

        game

    }


    fn from(&mut self) -> u32 {

        println!("Which piece do you want to move?");

        let mut row = 0;
        let mut column = 0;
        let mut place = 0;

        loop{

            let (row, column) = convert_input_to_row_column();

            place = ((row)*8)+column;
            

            match &self.board.squares[place as usize] {

                Some(_Piece) => {

                    let player_color = match self.player{

                        Player::WhitePlayer => Color::White,
                        Player::BlackPlayer => Color::Black,
                    };
        
                    match &self.board.squares[place as usize] {
                        Some(Piece) => {
        
                            if player_color == Piece.color{
                                println!("This is your piece!");
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
                    println!("There is no piece at this square! Select another one!");
                }
            }
        }

        place

    }


    fn to(&mut self, from: u32) -> u32 {

        let mut to: u32 = 0;

        let possible_moves: Vec<(u32, u32)> = Game::get_possible_moves(&self, from);

        if possible_moves.len() == 0 {

            println!("There are no possible moves for this piece. Choose another one");

            Game::from(self);

        }
        else {

            loop {
                
                println!("These are the possible moves:");

                for &(row, column) in &possible_moves {

                    let (letter, rank) = convert_row_column_to_output(row, column);

                    print!("{}", letter);
                    print!("{}", rank);
                    println!(" ");

                }

                println!("Choose one of these moves!");

                let chosen_move: (u32, u32) = convert_input_to_row_column();

                let mut move_in_list = false;

                for (row1, column1) in &possible_moves {

                    if chosen_move == (*row1, *column1) {

                        move_in_list = true;

                        let (to_row, to_column) = chosen_move;

                        let to = row_column_to_square(chosen_move);

                        Game::make_move(self, from, to);

                    }
                }

                if move_in_list == false {

                    println!("Choose one of the possible moves!");
                }
                else{
                    break;
                }

            

            
            }
        
        }

        to

    }
    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: u32, to: u32) {

        let swap = std::mem::replace(&mut self.board.squares[from as usize], None);

        self.board.squares[to as usize] = swap;
        
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
    pub fn get_possible_moves(&self, from: u32) -> Vec<(u32, u32)> {
      
        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        //Make the from into a vector with the row and column of the piece

        let (row, column) = square_to_row_column(from);

        let from_row_column = (row, column);


        match &self.board.squares[from as usize] {

            Some(piece) => {

                let piece_type = match piece.piece_type {
                    
                    PieceType::King => PieceType::King,
                    PieceType::Queen => PieceType::Queen,
                    PieceType::Bishop => PieceType::Bishop,
                    PieceType::Knight => PieceType::Knight,
                    PieceType::Rook => PieceType::Rook,
                    PieceType::Pawn => PieceType::Pawn,

                    };

                    if piece_type == PieceType::King {

                        possible_moves = Game::possible_moves_king(self, from_row_column);

                        println!("Its a king!");

                    }      
                    else if piece_type == PieceType::Knight {

                        possible_moves = Game::possible_moves_knight(&self, from_row_column);

                        println!("Its a knight!");
                    }
                    else if piece_type == PieceType::Rook {
                        
                        possible_moves = Game::possible_moves_rook(&self, from_row_column);

                        println!("Its a rook!");
                        
                    }
                    else if piece_type == PieceType::Queen {

                        possible_moves = Game::possible_moves_queen(&self, from_row_column);
                        println!("Its a queen!");
                    }
                    else if piece_type == PieceType::Bishop {
                        
                        possible_moves = Game::possible_moves_bishop(&self, from_row_column);
                        println!("Its a bishop");
                    }
                    else if piece_type == PieceType::Pawn {

                        println!("Its a pawn")
                    }
                }        
        
            None => {

                println!("There is no piece at this square!");
            }

        }

        let antal_moves = possible_moves.len();

        println!("{}", antal_moves);

        possible_moves

    }

    fn possible_moves_king(&self, from: (u32, u32)) -> Vec<(u32, u32)>{ //Possible move when the square is empty or there is an opponents piece there.

            //The king can move in each direction one step. If it is not at the edge of the board this is eight possible squares.
            //The kings move the same regardless of color

        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        let mut moves_on_the_board: Vec<(u32, u32)> = Vec::new();

        let moves = [

            (1, -1), (1, 0), (1, 1),

            (0, -1),         (0, 1),

            (-1, -1), (-1, 0), (-1, 1),

        ];
            
        let (mut row, mut column) = from;
        
            
        for (r, c) in moves.iter() {

            let (row1, column1) = ((row as i32) + *r, (column as i32) + *c);

            if -1 < row1 && row1 < 8 {

                if -1 < column1 && column1 < 8 {

                    moves_on_the_board.push((row1 as u32, column1 as u32));
                }
            }
        }

        for i in 0..moves_on_the_board.len() {

            let (row, column) = moves_on_the_board.get(i).unwrap();
            let square: u32 = ((row*8) + column).try_into().unwrap();

            match &self.board.squares[square as usize] {

                Some(_Piece) => {

                    let player_color = match self.player{

                        Player::WhitePlayer => Color::White,
                        Player::BlackPlayer => Color::Black,
                    };
                
                    match &self.board.squares[square as usize] {
                        Some(Piece) => {
                
                            if player_color != Piece.color{
                                    
                                possible_moves.push((*row, *column));

                            }

                        },
                        None => {

                            possible_moves.push((*row, *column));

                        }
                    }
                }

                None => {
                    possible_moves.push((*row, *column));
                }
            }
        }

        possible_moves

    }


    fn possible_moves_knight(&self, from: (u32, u32)) -> Vec<(u32, u32)> {

        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        let mut moves_on_the_board: Vec<(u32, u32)> = Vec::new();

        let moves: [(i32, i32); 8] = [

            (2, -1), (2, 1), 
            
            (1, -2), (1, 2),
                
            (-1, -2), (-1, 2), 
                
            (-2, -1), (-2, 1),

        ];
            
        let (mut row, mut column) = from;
        
            
        for (r, c) in moves.iter() {

            let (row1, column1) = ((row as i32) + *r, (column as i32) + *c);

            if -1 < row1 && row1 < 8 {

                if -1 < column1 && column1 < 8 {

                    moves_on_the_board.push((row1 as u32, column1 as u32));
                }
            }
        }

        for i in 0..moves_on_the_board.len() {

            let (row, column) = moves_on_the_board.get(i).unwrap();
            let square: u32 = ((row*8) + column).try_into().unwrap();

            match &self.board.squares[square as usize] {

                Some(_Piece) => {

                    let player_color = match self.player{

                        Player::WhitePlayer => Color::White,
                        Player::BlackPlayer => Color::Black,
                    };
            
                    match &self.board.squares[square as usize] {
                        Some(Piece) => {
            
                            if player_color != Piece.color{
                                    
                                possible_moves.push((*row, *column));

                            }

                        },
                        None => {

                            possible_moves.push((*row, *column));

                        }
                    }
                }

                None => {
                    possible_moves.push((*row, *column));
                }
            }
        }

        possible_moves

    }
    

    fn possible_moves_rook(&self, from: (u32, u32)) -> Vec<(u32, u32)> {

        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        let player_color = match self.player{

            Player::WhitePlayer => Color::White,
            Player::BlackPlayer => Color::Black,
        };

        let directions: [(i32, i32); 4] = [

            (1, 0), (-1, 0), (0, 1), (0, -1), //These are the directions for the rook

        ];
            
        let (row, column) = from;
            
        for (r, c) in directions.iter() {

            let (mut to_row, mut to_column) = ((row as i32) + *r, (column as i32) + *c);

            while -1 < to_row && to_row < 8 && -1 < to_column && to_column < 8 {

                let square = row_column_to_square((to_row as u32, to_column as u32));

                
                match &self.board.squares[square as usize] {

                    Some(Piece) => {
                    
                        if player_color == Piece.color{ //If we find the same players piece we cannot move anymore
                            break;
                        }
                        else {          // If we find the opponents piece we can capture it mut not move beyond that.
                            possible_moves.push((to_row as u32, to_column as u32));
                            break;
                        }
                        
        
                        },
                    None => { //If nothing is there we can move there
        
                        possible_moves.push((to_row as u32, to_column as u32));
        
                    }
                }

                to_row = (to_row as i32) + *r;
                to_column = (to_column as i32) + *c;
            }
        }

        return possible_moves;

    }

    fn possible_moves_bishop(&self, from: (u32, u32)) -> Vec<(u32, u32)> {

        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        let player_color = match self.player{

            Player::WhitePlayer => Color::White,
            Player::BlackPlayer => Color::Black,

        };

        let directions: [(i32, i32); 4] = [

            (1, 1), (1, -1), (-1, 1), (-1, -1), //These are the directions for the rook

        ];
            
        let (row, column) = from;        
            
        for (r, c) in directions.iter() {

            let (mut to_row, mut to_column) = ((row as i32) + *r, (column as i32) + *c);

            while -1 < to_row && to_row < 8 && -1 < to_column && to_column < 8 {

                let square = row_column_to_square((to_row as u32, to_column as u32));
                
                match &self.board.squares[square as usize] {

                    Some(Piece) => {
                    
                        if player_color == Piece.color{ //If we find the same players piece we cannot move anymore
                            break;
                        }
                        else {          // If we find the opponents piece we can capture it mut not move beyond that.
                            possible_moves.push((to_row as u32, to_column as u32));
                            break;
                        }
                        
        
                        },
                    None => { //If nothing is there we can move there
        
                        possible_moves.push((to_row as u32, to_column as u32));
        
                    }
                }

                to_row = (to_row as i32) + *r;
                to_column = (to_column as i32) + *c;
            }
        }

        return possible_moves;

    }
    

    fn possible_moves_queen(&self, from: (u32, u32)) -> Vec<(u32, u32)> {

        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        let player_color = match self.player{

            Player::WhitePlayer => Color::White,
            Player::BlackPlayer => Color::Black,
        };

        let directions: [(i32, i32); 8] = [

            (1, 0), (-1, 0), (0, 1), (0, -1), (1,1), (1,-1), (-1,1), (-1,-1), //These are the directions for the rook

        ];
            
        let (row, column) = from;
            
        for (r, c) in directions.iter() {

            let (mut to_row, mut to_column) = ((row as i32) + *r, (column as i32) + *c);

            while -1 < to_row && to_row < 8 && -1 < to_column && to_column < 8 {

                let square = row_column_to_square((to_row as u32, to_column as u32));

                match &self.board.squares[square as usize] {

                    Some(Piece) => {
                    
                        if player_color == Piece.color{ //If we find the same players piece we cannot move anymore
                            break;
                        }
                        else {          // If we find the opponents piece we can capture it mut not move beyond that.
                            possible_moves.push((to_row as u32, to_column as u32));
                            break;
                        }
                        
        
                        },
                    None => { //If nothing is there we can move there
        
                        possible_moves.push((to_row as u32, to_column as u32));
        
                    }
                }

                to_row = (to_row as i32) + *r;
                to_column = (to_column as i32) + *c;
            }
        }

        return possible_moves;

    }

    fn whose_turn(&self) -> Player {
        self.player
    }

    fn change_player(&mut self) {

        self.player = match self.player {

            Player::WhitePlayer => Player::BlackPlayer,
            Player::BlackPlayer => Player::WhitePlayer,
        }
    }
    
}

fn main() {
    let mut game = Game::new(); //Initializes a new game with the board set up in the initial position, game state as active and player as white

    println!("{:?}", game); //Prints the board

    let game_state = Game::get_game_state(&game);

    while game_state == GameState::InProgress{

        let turn = Game::whose_turn(&game);

        let move_from = Game::from(&mut game);

        let move_to = Game::to(&mut game, move_from);

        println!("{:?}", game);

        Game::change_player(&mut game);

    }

}








fn square_to_row_column(square: u32) -> (u32, u32) {

    let (row, column )  = (square/8, square%8);

    return (row, column);
}

fn row_column_to_square((row, column): (u32, u32)) -> u32{

    let mut square = 0;

    if row == 0 {
        
        square = column;
    }
    else {

        square = ((row*8) + column).try_into().unwrap();
    }

    return square;
}

fn convert_input_to_row_column() -> (u32, u32) { //Converts the user input of a3 to a row and a column

    let input = io::stdin();
    
    let mut place1 = input 
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();

    let characters: Vec<char> = place1.get(0).expect("REASON").chars().collect();

    let mut row = 0;
    let mut column = 0;

    if let Some(&letter) = characters.get(0) {

        match letter {

            'a' => column = 0,
            'b' => column = 1,
            'c' => column = 2,
            'd' => column = 3,
            'e' => column = 4,
            'f' => column = 5,
            'g' => column = 6,
            'h' => column = 7,
            _ => println!("This is not a letter that matches a column"),
        }
    }
    else {
        println!("Invalid character");
    }


    if let Some(&digit) = characters.get(1) {
        if let Some(number) = digit.to_digit(10) {
            row = number -1;
        } else {
            println!("Invalid character!");
        }
    } else {
        println!("Invalid character");
    }

    return (row, column)

}


fn convert_row_column_to_output(row: u32, column: u32) -> (char, u32){


    let row1 = row+1;
    let mut letter: char = ' ';
    

    match column {

        0 => letter = 'a',
        1 => letter = 'b',
        2 => letter = 'c',
        3 => letter = 'd',
        4 => letter = 'e',
        5 => letter = 'f',
        6 => letter = 'g',
        7 => letter = 'h',
        _ => (),
    }

    return (letter, row1)


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
                            PieceType::King => "W.K  ",
                            PieceType::Queen => "W.Q  ",
                            PieceType::Rook => "W.R  ",
                            PieceType::Bishop => "W.B  ",
                            PieceType::Knight => "W.Kn  ",
                            PieceType::Pawn => "W.P  ",
                        },
                        Color::Black => match piece.piece_type {
                            PieceType::King => "B.K  ",
                            PieceType::Queen => "B.Q  ",
                            PieceType::Rook => "B.R  ",
                            PieceType::Bishop => "B.B  ",
                            PieceType::Knight => "B.Kn  ",
                            PieceType::Pawn => "B.P  ",
                        },
                    };
                    board_string.push_str(piece_str);
                } else {

                    board_string.push(' ');
                    board_string.push(' ');
                    board_string.push('*');
                    board_string.push(' ');
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
    use crate::convert_input_to_row_column;
    use crate::square_to_row_column;

    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn conversion() {

        let conversion = square_to_row_column(10);
        assert_eq!(conversion, (1,2));
    }

    #[test]

    fn input_square() {

        let square = convert_input_to_row_column();
        assert_eq!(square, (0,0));
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