use macroquad::prelude::*;

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
    loop {
        clear_background(BLACK);

        draw_circle(0.0, 0.0, 20.0, WHITE);

        println!("fps: {}", get_fps());
        next_frame().await;
    }
}

#[cfg(test)]
#[allow(dead_code)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    // #[macroquad::test]
    fn fake_test() {
        assert_eq!(1, 1);
    }
}
