

use std::string::String;
use crate::Board;
pub struct User{
    name: String,
    board:Board
}
use crate::BOARD_WIDTH;
use crate::BOARD_HEIGHT;
use crate::PIECE_PER_USER;

impl User{

    pub fn new(user_name:&str)->User{

        println!("user created");
        User{
            name:String::from(user_name),
            board:User::create_user_board()
        }
    }
        
    fn create_user_board()->Board{
        fn is_cross(rowidx:usize,colidx:usize)->bool{
            if (rowidx == colidx) || (rowidx == (BOARD_HEIGHT-1-colidx)){
                true
            }else{
                false
            } 
        
        }
        let mut user_board = [[0;BOARD_WIDTH];BOARD_HEIGHT];
        user_board[0][0] = PIECE_PER_USER as i32;
        return user_board;
    }
    pub fn get_user_board_status(&self )->Board{
        return self.board
    }
}

