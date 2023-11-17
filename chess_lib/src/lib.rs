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
pub enum Player{
    WhitePlayer,
    BlackPlayer,
}

#[derive(Clone, PartialEq, Copy)]
pub enum Color{
    White,
    Black,
}


#[derive( Clone, PartialEq)]
pub enum PieceType{
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}


#[derive(Clone)]
pub struct Piece{
    color: Color,
    piece_type: PieceType,
}


impl Piece {

    pub fn new(color: Color, piece_type: PieceType) -> Piece {
        
        let piece = Piece {
            color,
            piece_type,
        };
        
        piece
    }

    pub fn get_color(&self) -> Color{
        self.color
    }
}


#[derive(Clone)]
pub struct Board {
    
    squares: Vec<Option<Piece>>,
}

impl Board { //Implements the board struct

    pub fn new() -> Self { //This initializes a new board with all the pieces in their correct spots
        let mut squares = Vec::with_capacity(64); // Create a vector with a capacity of 64 squares
        // Initialize the vector with None values for each square
        for _ in 0..64 {
            squares.push(None);
        }
        //Puts all the pieces in their coorect starting spots
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

    pub fn get_squares(&self) -> &Vec<Option<Piece>> {
        &self.squares
    }

}


#[derive(Clone)]
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
    
    pub fn get_game(self) -> (Game) {

        return self;
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }


    pub fn from(&mut self) -> u32 { //This is a function that will let the user choose which piece to move by entering the coordinates for the square the piece is on

        println!("Which piece do you want to move?");

        let mut place = 0;

        loop{

            let (row, column) = convert_input_to_row_column();

            place = ((row)*8)+column;
            

            match &self.board.squares[place as usize] { //We check if there is a piece at the given square and weather it belongs to the user. 

                Some(_Piece) => {

                    let player_color = match self.player{ //First we note the players color

                        Player::WhitePlayer => Color::White,
                        Player::BlackPlayer => Color::Black,
                    };
        
                    match &self.board.squares[place as usize] { //If the color of the piece matches the color of the player the piece belongs to them

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

        place //Returns the square number for the piece the user wants to move.

    }


    pub fn to(&mut self, from: u32) -> u32 { //This function will let the user pick from the available moves for the piece they want to move. If there are no moves they have to pick again

        let mut to: u32 = 0;


        let white = match self.player {

            Player::WhitePlayer => true,
            Player::BlackPlayer => false,
        };

        let possible_moves: Vec<(u32, u32)> = Game::get_possible_moves(&self, from, white); //This gives us all the possible moves for the piece the player is trying to move

        let mut possible_moves_after_check = Vec::new();

        for &(row, column) in &possible_moves { //This function will pass all the moves to the legal_move function to see if the moves are legal or not.

            let (from_row, from_column) = square_to_row_column(from);

            let legal_move = self.legal_move((row, column), (from_row, from_column));
            
            if legal_move == true { //If the move is legal we can add it to the list

                possible_moves_after_check.push((row, column));

            }
        }

        if possible_moves_after_check.len() == 0 { //If there are no legal moves the player has to choose again

            println!("There are no possible moves for this piece. Choose another one");

            Game::from(self);

        }
        else {

            loop {
                
                println!("These are the possible moves:");

                for &(row, column) in &possible_moves_after_check {

                    let (letter, rank) = convert_row_column_to_output(row, column);

                    //Prints out all the possible moves from which the user can choose which one they want.
                    print!("{}", letter);
                    print!("{}", rank);
                    println!(" ");

                }

                println!("Choose one of these moves!");

                let chosen_move: (u32, u32) = convert_input_to_row_column();

                let mut move_in_list = false;

                for (row1, column1) in &possible_moves_after_check { //First checks so the move the user picked is in the list of posible moves. If it is it will move the piece

                    if chosen_move == (*row1, *column1) {

                        move_in_list = true;

                        let to = row_column_to_square(chosen_move);

                        Game::make_move(self, from, to);

                    }
                }

                if move_in_list == false { //If the move wasnt in the list of possible moves the user has to pick again

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
    pub fn make_move(&mut self, from: u32, to: u32) { //This funciton will move the piece.

        let swap = std::mem::replace(&mut self.board.squares[from as usize], None); //We store the piece we are trying to move in swap and leave the source as empty

        self.board.squares[to as usize] = swap; //The destination square gets the piece of the source square. This will override anything at teh square so it also works well for capturing.
        
    }

    /// (Optional but recommended) Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState { //Gets the state of the game
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Implement en passant and castling.
    pub fn get_possible_moves(&self, from: u32, white: bool) -> Vec<(u32, u32)> { //This function will first determine which type of piece we have and then call the specific function for getting move of that piecetype
        
        let player_color = match white { //Because we cannot use a private enum Color as parameter for a public function we instead have a boolean which tells us if the player is white or not. 

            true => Color::White,
            false => Color::Black,
        };

        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        //Make the from into a vector with the row and column of the piece

        let (row, column) = square_to_row_column(from);

        let from_row_column = (row, column);


        match &self.board.squares[from as usize] { //Determines the piece type on the square the piece the player is trying to move.

            Some(piece) => {

                let piece_type = match piece.piece_type {
                    
                    PieceType::King => PieceType::King,
                    PieceType::Queen => PieceType::Queen,
                    PieceType::Bishop => PieceType::Bishop,
                    PieceType::Knight => PieceType::Knight,
                    PieceType::Rook => PieceType::Rook,
                    PieceType::Pawn => PieceType::Pawn,

                    };

                    //Depending on which piece type it is differnt functions are called.
                    if piece_type == PieceType::King {

                        possible_moves = self.possible_moves_king(from_row_column, player_color);

                    }      
                    else if piece_type == PieceType::Knight {

                        possible_moves = self.possible_moves_knight(from_row_column, player_color);

                    }
                    else if piece_type == PieceType::Rook {
                        
                        possible_moves = self.possible_moves_rook(from_row_column, player_color);
                        
                    }
                    else if piece_type == PieceType::Queen {

                        possible_moves = self.possible_moves_queen(from_row_column, player_color);

                    }
                    else if piece_type == PieceType::Bishop {
                        
                        possible_moves = self.possible_moves_bishop(from_row_column, player_color);
                       
                    }
                    else if piece_type == PieceType::Pawn {

                        possible_moves = self.possible_moves_pawn(from_row_column, player_color);
                        
                    }
                }        
        
            None => {

                println!("There is no piece at this square!");
            }

        }

        possible_moves //Returns all the possible moves

    }

    pub fn possible_moves_king(&self, from: (u32, u32), player_color: Color) -> Vec<(u32, u32)>{ //Determines the possible moves for the king

            //The king can move in each direction one step. If it is not at the edge of the board this is eight possible squares.
            //The kings move the same regardless of color

        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        let mut moves_on_the_board: Vec<(u32, u32)> = Vec::new();

        let moves = [ //The king always has 8 different moves it could make (not counting castling). These are the changes in row and column for every move

            (1, -1), (1, 0), (1, 1),

            (0, -1),         (0, 1),

            (-1, -1), (-1, 0), (-1, 1),

        ];
            
        let (mut row, mut column) = from;
        
            
        for (r, c) in moves.iter() { //First we check that the move doesnt take the piece outside of the board.

            let (row1, column1) = ((row as i32) + *r, (column as i32) + *c);

            if -1 < row1 && row1 < 8 {

                if -1 < column1 && column1 < 8 {

                    moves_on_the_board.push((row1 as u32, column1 as u32));
                }
            }
        }

        for i in 0..moves_on_the_board.len() { //This will check so the piece cant stand on top of another piece of the same color, but it can still capture the opponents piece.

            let (row, column) = moves_on_the_board.get(i).unwrap();
            let square: u32 = ((row*8) + column).try_into().unwrap();

            match &self.board.squares[square as usize] { //Chekc if the square is empty or not

                Some(_Piece) => {

                    match &self.board.squares[square as usize] { //If there is a piece we can still move there if the piece  color and player color do not match
                        Some(Piece) => {
                
                            if player_color != Piece.color{
                                    
                                possible_moves.push((*row, *column));

                            }

                        },
                        None => { //If the square is empty we can move there

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


    pub fn possible_moves_knight(&self, from: (u32, u32), player_color: Color) -> Vec<(u32, u32)> { //Returns all the possible moves of the knight

        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        let mut moves_on_the_board: Vec<(u32, u32)> = Vec::new();

        let moves: [(i32, i32); 8] = [ //Like the king. The knight also has pre determined moves it could make regardless of color

            (2, -1), (2, 1), 
            
            (1, -2), (1, 2),
                
            (-1, -2), (-1, 2), 
                
            (-2, -1), (-2, 1),

        ];

        //Except for the specific moves the piece can make this function is almost the exact same as the one for the king moves
            
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
    

    pub fn possible_moves_rook(&self, from: (u32, u32), player_color: Color) -> Vec<(u32, u32)> { //Returns all the possible moves of the rook

        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        let directions: [(i32, i32); 4] = [

            (1, 0), (-1, 0), (0, 1), (0, -1), //These are the directions for the rook. Instead of specific move the rook can move in certain directions. These are the changes in row and column for one step in each direction

        ];
            
        let (row, column) = from;
            
        for (r, c) in directions.iter() { //We will see how far we can move in each direction. It will "move" the piece as far as possible in all four direction, one move at a time and check if the move is possible or not.

            let (mut to_row, mut to_column) = ((row as i32) + *r, (column as i32) + *c);

            while -1 < to_row && to_row < 8 && -1 < to_column && to_column < 8 {

                let square = row_column_to_square((to_row as u32, to_column as u32));

                
                match &self.board.squares[square as usize] { //The rook can move in any direction until it reaches the border or another piece. This function sees how far it can move

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

                to_row = (to_row as i32) + *r; //After every move, if the square was empty we continue moving in the same direction. We do this until we reach another piece or the edge of the board.
                to_column = (to_column as i32) + *c;
            }
        }

        return possible_moves;

    }

    pub fn possible_moves_bishop(&self, from: (u32, u32), player_color: Color) -> Vec<(u32, u32)> { //Returns all the possible moves for the bishop

        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        let directions: [(i32, i32); 4] = [

            (1, 1), (1, -1), (-1, 1), (-1, -1), //Similarly to the rook, the bishop can move until another piece or the edge of the board blocks it in four directions. The only difference is that it moves idagonally.

        ];
        
        //Since the moving is so similar to the rook the rest of the function is basically the same as the rook
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
    

    pub fn possible_moves_queen(&self, from: (u32, u32), player_color: Color) -> Vec<(u32, u32)> { //Returns all the possible moves for the queen

        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        let directions: [(i32, i32); 8] = [

            (1, 0), (-1, 0), (0, 1), (0, -1), (1,1), (1,-1), (-1,1), (-1,-1), //The queen has all the combined directions of the rook and the bishop

        ];
        
        //This function is also the same as the last two, just the directions are different.
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


    pub fn possible_moves_pawn(&self, from: (u32, u32), player_color: Color) -> Vec<(u32, u32)> { //Returns all the possible moves of the pawn

        //The pawn is difficult since it has different directions depending on the player since pawns can only move relatively forward.
        let mut possible_moves: Vec<(u32, u32)> = Vec::new();

        let mut moves: [(i32, i32); 4] = [(0,0), (0,0), (0,0),(0,0),]; 

        if player_color == Color::Black { //Depending on the player, the pawn has different moves.

            moves = [

                (-1,1), (-1,0), (-1,-1), (-2, 0), //There are five possible moves at most for a pawn. Forward one step, forward two steps if it hasnt moved before, and to each side if a piece of the opposing color is occupying that square
            ]
        }
        else {

            moves = [

                (1,1), (1,0), (1,-1), (2,0),
            ]
        }
          
        let (mut row, mut column) = from;
        
            
        for (r, c) in moves.iter() { //Determines the possible moves for the pawn

            let (row1, column1) = ((row as i32) + *r, (column as i32) + *c);

            if -1 < row1 && row1 < 8 {

                if -1 < column1 && column1 < 8 { //Makes sure we stay on the board

                    let square: u32 = row_column_to_square((row1 as u32, column1 as u32)); 

                    match &self.board.squares[square as usize] { //We check if a piece is standing where we want to move

                        Some(Piece) => {
            
                            if player_color != Piece.color{ // If an opponents piece is standing at the square the pawn wants to move to we can only go there if it is a diagonal square since the pawns only capture diagonally

                                if *c != 0 { //The piece can only capture to the sides

                                    possible_moves.push((row1 as u32, column1 as u32));

                                }
                            }

                        }

                        None => {

                            
                            if *r == 2 { //The piece can move two steps forward if it hasnt moved before
                                if row == 1 {
                                    possible_moves.push((row1 as u32, column1 as u32));
                                }
                                else if row == 6{
                                    possible_moves.push((row1 as u32, column1 as u32));   //We can use the rows 1 and 6 since the opposing players pieces cannot move two squares forward from the second to last rank since it will move them off the board      
                                }
                            }
                            else {
                                if *c == 0 { //The piece can only move straight if there is no other piece there

                                    possible_moves.push((row1 as u32, column1 as u32));
                                }
                            }
                            
                        }
                    }
                
                }
            }

        }

        possible_moves

        //As a sidenote. I just noticed while writing the comments that it might be possible to move a pawn two squares if there is a piece standing one square in front. Havent tested this but it might be possible

    }

    pub fn legal_move(&mut self, to: (u32, u32), from: (u32,u32)) -> bool { //Sees if the move is legal. It is legal if the move does not lead to the players king being in check

        let mut simulation = self.clone(); //We simulate the move to see if it will make the players own king be checked.
        
        let from_square = row_column_to_square((from.0, from.1));
        let to_square = row_column_to_square((to.0, to.1));
        
        simulation.make_move(from_square, to_square);

        let player_color = match self.player { 

            Player::BlackPlayer => Color::Black,
            Player::WhitePlayer => Color::White,
        };

        let mut king_square = 0;

        for i in 0..64 { //We need to find the square where the players king is

            match &simulation.board.squares[i as usize] { //Check every single square and if the king is there we assign that particular square as the king square

                Some(piece) => {

                    let piece_type = match piece.piece_type {
                        
                        PieceType::King => PieceType::King,
                        PieceType::Queen => PieceType::Queen,
                        PieceType::Bishop => PieceType::Bishop,
                        PieceType::Knight => PieceType::Knight,
                        PieceType::Rook => PieceType::Rook,
                        PieceType::Pawn => PieceType::Pawn,
        
                        };

                    if player_color == piece.color && piece_type == PieceType::King { //This is the players king

                        king_square = i;
                            
                    }

                }
                None => {
                }
            }
            
        }

        let mut possible_moves = Vec::new();

        for i in 0..64 { //We check the moves of all the opponents pieces and if they can move to the square of the king that means the move is illegal

            match &simulation.board.squares[i as usize] {

                Some(piece) => {
    
                    if player_color != piece.color{

                        let opponent_white = match self.player { //This is the reason all the get moves functions have a color. We need to be able to call the functions for the players own pieces as well as the opponents pieces.

                            Player::WhitePlayer => false,
                            Player::BlackPlayer => true,
                        };

                        possible_moves = simulation.get_possible_moves(i, opponent_white); //Gets all the moves of the opponents pieces
                        
                        for (row, column) in &possible_moves {

                            let move_square = row_column_to_square((*row, *column));

                            if move_square == king_square { //If a piee can attack the king it is an illegal move

                                return false;
                                
                            }
                            
                        }
                    }

                }
            

                None => {

               }
            }
        }

        return true;

    }
    
    pub fn checkmate(&mut self) -> bool { //Sees if the player is in checkmate
        
        let player_color = match self.player {

            Player::BlackPlayer => Color::Black,
            Player::WhitePlayer => Color::White,
        };

        let mut king_square = 0;

        for i in 0..64 { //We need to find the square where the king is

            match &self.board.squares[i as usize] { //Check every single square and if the king is there we assign that particular square as the king square

                Some(Piece) => {

                    let piece_type = match Piece.piece_type {
                        
                        PieceType::King => PieceType::King,
                        PieceType::Queen => PieceType::Queen,
                        PieceType::Bishop => PieceType::Bishop,
                        PieceType::Knight => PieceType::Knight,
                        PieceType::Rook => PieceType::Rook,
                        PieceType::Pawn => PieceType::Pawn,
        
                        };

                    if player_color == Piece.color && piece_type == PieceType::King { //This is the players king

                        king_square = i;
                            
                    }

                }
                None => {
                }
            }
            
        }

        let mut possible_moves = Vec::new();
        let mut all_moves = Vec::new();

        for i in 0..64 { //We check all the moves that the player can make and if there are no legal moves the player is in checkmate
            //This for loop will go through all the players pieces and add all of their possible moves to the possible_moves vector. It will then check if the moves are legal and if they are the are added to the all_moves vector

            match &self.board.squares[i as usize] {

                Some(Piece) => {
    
                    if player_color == Piece.color{

                        let white = match self.player {

                            Player::WhitePlayer => true,
                            Player::BlackPlayer => false,
                        };

                        possible_moves = self.get_possible_moves(i, white);
                        
                        for (row, column) in &possible_moves {

                            let move_square = row_column_to_square((*row, *column));

                            let from = square_to_row_column(i);
                        
                            let legal_move = self.legal_move((*row, *column), from);
                            
                            if legal_move == true {

                                all_moves.push((*row, *column));
                            }
                        }
                    }

                }
            

                None => {

               }
            }
        }

        if all_moves.len() == 0 { //This means there are no possible moves for the player, they are in checkmate
            println!("You are in checkmate!");
            return true;
        }
        else {
            return false;
        }
    }

    pub fn whose_turn(&self) -> Player {
        
        self.player
    }

    pub fn change_player(&mut self) {

        self.player = match self.player {

            Player::WhitePlayer => Player::BlackPlayer,
            Player::BlackPlayer => Player::WhitePlayer,
        }
    }
    
}

pub fn main() {
    let mut game = Game::new(); //Initializes a new game with the board set up in the initial position, game state as active and player as white

    println!("{:?}", game); //Prints the board in its starting position

    let game_state = Game::get_game_state(&game); //Game state is in progress since it is set to that when we initialize the game

    while game_state != GameState::GameOver { //As long as the game is not game over we can play. Every time the loop restarts it is a new turn

        let checkmate = Game::checkmate(&mut game); //Sees if you are in checkmate

        if checkmate == true { //If you are in checkmate the game ends
            break;
        }

        let turn = Game::whose_turn(&game); //Lets us know whose turn it is

        println!("It is {:?}'s turn", turn); //Prints whose turn it is

        let move_from = Game::from(&mut game); //Lets the player choose which piece to move

        let move_to = Game::to(&mut game, move_from); //Let sthe player choose where to move the piece and actually moves it.

        println!("{:?}", game); //Prints the new positions of all the pieces

        Game::change_player(&mut game); //Changes the player

    }

    println!("Game over!"); //After the game is over we print this

}








pub fn square_to_row_column(square: u32) -> (u32, u32) { //Goes from the number notation ro the row column notation for the square

    let (row, column )  = (square/8, square%8);

    return (row, column);
}

pub fn row_column_to_square((row, column): (u32, u32)) -> u32{ //Goes from row column notation to number notation for the square

    let mut square = 0;

    if row == 0 {
        
        square = column;
    }
    else {

        square = ((row*8) + column).try_into().unwrap();
    }

    return square;
}

pub fn convert_input_to_row_column() -> (u32, u32) { //Converts the user input of ""a3" to a row and a column

    let input = io::stdin(); 
    
    let mut place1 = input //Takes the users input and stores in a string
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap())
        .collect::<Vec<String>>();

    let characters: Vec<char> = place1.get(0).expect("REASON").chars().collect(); //Splits the string into two characters

    let mut row = 0;
    let mut column = 0;

    if let Some(&letter) = characters.get(0) { //Pattern matches the character to columns

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


    if let Some(&digit) = characters.get(1) { //We check so the index is correctly given. Inputs like os wont break the program
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


pub fn convert_row_column_to_output(row: u32, column: u32) -> (char, u32){ //Convers the row column notation to an output like a3


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