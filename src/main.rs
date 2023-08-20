extern crate good_web_game as ggez;

use ggez::cgmath::Point2;
use ggez::graphics;
use ggez::{event::EventHandler, miniquad, Context, GameResult};

fn main() -> GameResult<()> {
    ggez::start(ggez::conf::Conf::default(), |mut context, quad_ctx| {
        Box::new(MainState::new(&mut context, quad_ctx).unwrap())
    })
}

struct MainState {
    count: u32,
}

impl MainState {
    pub fn new(_ctx: &mut Context, _quad_ctx: &mut miniquad::GraphicsContext) -> GameResult<Self> {
        let state = MainState { count: 1 };

        Ok(state)
    }
}

impl EventHandler for MainState {
    fn update(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::GraphicsContext,
    ) -> GameResult {
        self.count += 1;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        let count_dest = Point2::new(10.0, 10.0);
        let count_str = format!("Counter: {}", self.count);
        let count_display = graphics::Text::new((count_str, graphics::Font::default(), 32.0));

        graphics::draw(
            ctx,
            quad_ctx,
            &count_display,
            (count_dest, 0.0, graphics::Color::WHITE),
        )?;

        Ok(())
    }
}
