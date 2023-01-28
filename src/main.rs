use std::collections::LinkedList;

use gtk::{prelude::*, DrawingArea};
use gtk::{Application,ApplicationWindow};
use gtk::Button;
use gtk::cairo::{Context};

mod user;
mod board;
use crate::board::Board;
use crate::board::GameBoard;
use crate::user::User;
const APP_ID:&str = "org.gtk.hello_world1";
const INITIAL_WINDOW_WIDTH:i32 = 720;
const INITIAL_WINDOW_HEIGHT:i32 = 480;
const INITIAL_WINDOW_TITLE:&str = "Hello, world!";
const DEFAULT_BUTTON_WIDTH:i32 = 120;
const DEFAULT_BUTTON_HEIGHT:i32 = 40;
const PI:f64 = 3.14;
pub const USER_COUNT:usize = 4;
pub const PIECE_PER_USER:usize = 4;
pub const BOARD_WIDTH:usize = 5;
pub const BOARD_HEIGHT:usize = 5;
pub const BOARD_IMAGE_WIDTH:usize = 300;
pub const BOARD_IMAGE_HEIGHT:usize = 300;
pub const BOARD_SPACE_WIDTH:usize = 30;
pub const BOARD_SPACE_HEIGHT:usize = 30;
pub const BOARD_LINE_WIDTH:usize = 2;

fn main(){

    // INITIALIZE
    let game_board:GameBoard = GameBoard::new();
   let mut userList:LinkedList<User> = LinkedList::new();
    for user in 0..USER_COUNT {
        let user = User::new(("Player ".to_owned()+user.to_string().as_str()).as_str());
        userList.push_back(user); 
    }
    game_board.generate_space();
    
    


    println!("Hello, world!");
    println!("{}",game_board.board.len()); 

    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();

}


fn build_button()->Button{
    let button = Button::builder()
        .label("press me")
        .width_request(DEFAULT_BUTTON_WIDTH)
        .height_request(DEFAULT_BUTTON_HEIGHT)
        .margin_top(12)
        .margin_start(12)
        .build();
    return button;
}
fn build_drawing_area()->DrawingArea{
    let drawing_area = DrawingArea::builder()
        .width_request(BOARD_IMAGE_WIDTH as i32)
        .height_request(BOARD_IMAGE_HEIGHT as i32)
        .margin_top(16)
        .margin_bottom(16)
        .margin_start(16)
        .margin_end(16)
        .build();
    drawing_area.set_draw_func(|area:&DrawingArea,cr:&Context,width:i32,height:i32|{
        //cr.set_dash(&[3., 2., 1.], 1.);
        //assert_eq!(cr.get_dash(), (vec![3., 2., 1.], 1.));

        cr.scale(500f64, 500f64);

        cr.set_source_rgb(250.0/255.0, 224.0/255.0, 55.0/255.0);
        cr.paint();

        cr.set_line_width(0.05);

        // border
        cr.set_source_rgb(0.3, 0.3, 0.3);
        cr.rectangle(0.0, 0.0, 1.0, 1.0);
        cr.stroke();

        cr.set_line_width(0.03);

        // draw circle
        cr.arc(0.5, 0.5, 0.4, 0.0, PI * 2.);
        cr.stroke();
    });
    drawing_area

}
fn build_ui(app:&Application){
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(INITIAL_WINDOW_WIDTH)
        .default_height(INITIAL_WINDOW_HEIGHT)
        .title(INITIAL_WINDOW_TITLE)
        .child(&build_drawing_area())
        //.child(&build_button())
        .build();
    window.present();
}
