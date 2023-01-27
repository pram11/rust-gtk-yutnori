use crate::BOARD_WIDTH;
use crate::BOARD_HEIGHT;


pub type Board = [[i32;BOARD_WIDTH];BOARD_HEIGHT];
pub struct GameBoard{
    pub board:Board
}

impl GameBoard{
    pub fn new()->GameBoard{
        GameBoard{
            board:GameBoard::create_board()
        }
    }
    fn create_board()->Board{
        fn is_cross(rowidx:usize,colidx:usize)->bool{
            if (rowidx == colidx) || (rowidx == (BOARD_HEIGHT-1-colidx)){
                true
            }else{
                false
            }
        }

        if BOARD_WIDTH != BOARD_HEIGHT {
            panic!("cannot create board");
        }else {
            let mut board_array = [[0; BOARD_WIDTH ];BOARD_HEIGHT ];
            
            for rowidx in 0..board_array.len(){
                if (rowidx ==0) || (rowidx==BOARD_HEIGHT-1){
                    board_array[rowidx] = [1;BOARD_HEIGHT];
                }else{
                
                    board_array[rowidx][0] = 1;
                    board_array[rowidx][board_array[rowidx].len()-1]=1;
                    for colidx in 0..board_array[rowidx].len(){

                        if is_cross(rowidx,colidx){
                            board_array[rowidx][colidx] = 1;
                        }
                    }
                }

                
            }

            //test for row in board_array{
            for row in board_array{

                for item in row{
                    print!("{}",item);
                }
                println!();
            }
            return board_array;
        }
        
    }
}
 
