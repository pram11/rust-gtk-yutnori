use std::collections::LinkedList;

use gtk::prelude::*;
use gtk::{Application,ApplicationWindow};
use gtk::Button;


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
fn build_ui(app:&Application){
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(INITIAL_WINDOW_WIDTH)
        .default_height(INITIAL_WINDOW_HEIGHT)
        .title(INITIAL_WINDOW_TITLE)
        .child(&build_button())
        .build();
    window.present();
}
