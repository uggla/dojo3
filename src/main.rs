mod screen;
mod stars;
mod text;

use macroquad::prelude::*;
use stars::Starfield;
use text::Text;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dojo intro".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(miniquad::date::now() as u64);

    let mut starfield = Starfield::new();

    let mut text = Text::new("RUST CODING DOJO 2024", 70)
        .speed(8.0)
        .sinus(true)
        .color(RED)
        .rainbow(true)
        .build();

    loop {
        clear_background(BLACK);

        starfield.add_star();
        starfield.update();
        starfield.draw();

        text.draw();
        println!("fps: {}", get_fps());
        next_frame().await;
    }
}
