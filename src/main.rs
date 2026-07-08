

use macroquad::prelude::*;


fn window_conf()->Conf{
    Conf{
        window_title : "Silent Guardian".to_owned(),
        window_height : 720,
        window_width : 1280,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main(){
    loop {
        clear_background(Color::from_rgba(26,29,41,255));
        next_frame().await;
    }
}

