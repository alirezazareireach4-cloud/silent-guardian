

use macroquad::prelude::*;

const APP_NAME: &str = "Silent Guardian";
const APP_SUBTITLE: &str = "Silent Emergency Alert System";
const APP_VERSION: &str = "v1.0.0";

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
        let width = screen_width();
        
        let title = measure_text(APP_NAME, None, 50, 1.0);

        draw_text(
            APP_NAME,
            (width - title.width) / 2.0,
            80.0,
            50.0,
            WHITE,
        );
        
        let subtitle = measure_text(APP_SUBTITLE, None, 28, 1.0);

        draw_text(
            APP_SUBTITLE,
            (width - subtitle.width) / 2.0,
            120.0,
            28.0,
            GRAY,
        );
                
        let version = measure_text(APP_VERSION, None, 24, 1.0);

        draw_text(
            APP_VERSION,
            (width - version.width) / 2.0,
            150.0,
            24.0,
            SKYBLUE,
        );

        next_frame().await;
    }
}

