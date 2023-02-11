use std::collections::LinkedList;

use gtk::{prelude::*, DrawingArea};
use gtk::{Application,ApplicationWindow};
use gtk::Button;
use gtk::cairo::{Context};
use json;

mod board;
use crate::board::Board;
const APP_ID:&str = "org.gtk.hello_world1";
const INITIAL_WINDOW_WIDTH:i32 = 720;
const INITIAL_WINDOW_HEIGHT:i32 = 480;
const INITIAL_WINDOW_TITLE:&str = "Hello, world!";
const DEFAULT_BUTTON_WIDTH:i32 = 120;
const DEFAULT_BUTTON_HEIGHT:i32 = 40;
const PI:f64 = 3.1415924771;
const BOARD_IMAGE_WIDTH:u32 = 600;
const BOARD_IMAGE_HEIGHT:u32 = 600;
fn main(){
    // test config file from /res/config.json
    let file = include_bytes!("../res/config.json");
    let file_string = String::from_utf8_lossy(file).to_string();
    let parsed = json::parse(&*file_string).unwrap();
    Board::from_json_string(parsed.to_string());
    // INITIALIZE
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    // app.run();

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
