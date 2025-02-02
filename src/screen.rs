use macroquad::prelude::*;

pub trait Centerable {
    fn centered(&self) -> Vec2;
}

impl Centerable for Vec2 {
    fn centered(&self) -> Vec2 {
        Vec2::new(
            self.x + screen_width() / 2.0,
            self.y + screen_height() / 2.0,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::screen::Centerable;
    use macroquad::prelude::*;

    #[macroquad::test]
    async fn test_centered() {
        let pos = Vec2::new(0.0, 0.0);
        assert_eq!(
            pos.centered(),
            Vec2::new(screen_width() / 2.0, screen_height() / 2.0)
        );
    }
}
