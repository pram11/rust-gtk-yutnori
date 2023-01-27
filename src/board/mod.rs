use crate::BOARD_WIDTH;
use crate::BOARD_HEIGHT;
use crate::BOARD_IMAGE_WIDTH;
use crate::BOARD_IMAGE_HEIGHT;
use crate::BOARD_SPACE_WIDTH;
use crate::BOARD_SPACE_HEIGHT;
use image::{ImageBuffer,RgbImage};

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
            }else{ false
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
        
    fn xspace_between_boardspace(&self)->usize{
            
        let corewidth = BOARD_IMAGE_WIDTH - BOARD_SPACE_WIDTH;
        (corewidth / BOARD_WIDTH)
    }
    fn yspace_between_boardspace(&self)->usize{
        let coreheight = BOARD_IMAGE_HEIGHT - BOARD_SPACE_HEIGHT;
        (coreheight / BOARD_HEIGHT)
    }

        fn calculate_space_center_coordinate(&self)->[[[usize;BOARD_HEIGHT];BOARD_WIDTH];2] { 
            
            let mut coordinate:[[[usize;BOARD_HEIGHT];BOARD_WIDTH];2] = [[[0;BOARD_HEIGHT];BOARD_WIDTH];2]; 
            for idx in 0..coordinate.len(){
                for widx in 0..coordinate[idx].len(){ 
                    for hidx in 0..coordinate[idx][widx].len(){ 
                        let x = (BOARD_SPACE_WIDTH/2)+(widx*self.xspace_between_boardspace() as usize);
                        let y = (BOARD_SPACE_HEIGHT/2) + (hidx*self.yspace_between_boardspace() as usize);
                        coordinate[0][widx][hidx] = x;
                        coordinate[1][widx][hidx] = y;
                    }
                } 
            }
            coordinate
    }

    pub fn generate_space(&self){
        let mut image_buffer:RgbImage = RgbImage::new(BOARD_IMAGE_WIDTH as u32 ,BOARD_IMAGE_HEIGHT as u32);
        println!("generate space");
        let coordinate_array = self.calculate_space_center_coordinate();
        for idx in 0..coordinate_array.len(){
            for widx in 0..coordinate_array[idx].len(){
                println!();
                for hidx in 0..coordinate_array[idx][widx].len(){
                    print!("{0}-{1} ",coordinate_array[0][widx][hidx],coordinate_array[1][widx][hidx]);
                }
            }
        }
        


        
    }


}
 
