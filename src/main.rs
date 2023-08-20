extern crate good_web_game as ggez;

use ggez::cgmath::Point2;
use ggez::event::EventHandler;
use ggez::event::KeyMods;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::DrawParam;
use ggez::graphics::TextFragment;
use ggez::input::MouseButton;
use ggez::miniquad;
use ggez::miniquad::KeyCode;
use ggez::Context;
use ggez::GameResult;

fn main() -> GameResult<()> {
    ggez::start(ggez::conf::Conf::default(), |mut context, quad_ctx| {
        Box::new(Main::new(&mut context, quad_ctx).unwrap())
    })
}

struct Main {
    count: u32,
    mouse: MouseState,
    show_debug: bool,
}

#[derive(Default)]
struct MouseState {
    down: bool,
    x: f32,
    y: f32,
}

impl Main {
    pub fn new(_ctx: &mut Context, _quad_ctx: &mut miniquad::GraphicsContext) -> GameResult<Self> {
        let state = Main {
            count: 1,
            mouse: MouseState::default(),
            show_debug: false,
        };

        Ok(state)
    }
}

fn draw_debug_text<const N: usize>(
    ctx: &mut Context,
    quad_ctx: &mut miniquad::GraphicsContext,
    lines: [impl Into<TextFragment>; N],
) -> GameResult {
    if lines.is_empty() {
        return Ok(());
    }

    let (width, height) = quad_ctx.screen_size();

    let margin = 20.0;
    let padding = 8.0;
    let line_height = 5.0;
    let font_size = 18.0;

    let text_color = Color::WHITE;
    let background_color = Color::from_rgba(128, 0, 0, 128);

    let rect_height =
        padding * 2.0 + font_size * lines.len() as f32 + line_height * (lines.len() as f32 - 1.0);
    let rect_width = width - margin * 2.0;
    let rect_x = margin;
    let rect_y = height - margin - rect_height;

    let rect = graphics::Rect::new(rect_x, rect_y, rect_width, rect_height);

    let mesh =
        graphics::Mesh::new_rectangle(ctx, quad_ctx, DrawMode::fill(), rect, background_color)?;

    graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;

    for (i, line) in lines.into_iter().enumerate() {
        let position = Point2::new(
            rect_x + padding,
            rect_y + padding + (font_size + line_height) * i as f32,
        );
        let text = graphics::Text::new((line, graphics::Font::default(), font_size));

        graphics::draw(ctx, quad_ctx, &text, (position, 0.0, text_color))?;
    }
    Ok(())
}

impl EventHandler for Main {
    fn update(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::GraphicsContext,
    ) -> GameResult {
        self.count += 1;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, Color::BLACK);

        let board_margin = 20.0;
        let tile_size = 60.0;
        let tile_color_a = Color::from_rgb(140, 80, 50);
        let tile_color_b = Color::from_rgb(80, 30, 20);
        let tile_highlight_width = 2.0;

        let mouse_x = ((self.mouse.x - board_margin) / tile_size) as i32;
        let mouse_y = ((self.mouse.y - board_margin) / tile_size) as i32;

        let tile_rect = |x: i32, y: i32| -> graphics::Rect {
            graphics::Rect::new(
                x as f32 * tile_size + board_margin,
                y as f32 * tile_size + board_margin,
                tile_size,
                tile_size,
            )
        };

        for x in 0..8 {
            for y in 0..8 {
                let color = if (x + y) % 2 == 0 {
                    tile_color_a
                } else {
                    tile_color_b
                };

                let mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    quad_ctx,
                    DrawMode::fill(),
                    tile_rect(x, y),
                    color,
                )?;
                graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;
            }
        }

        if (0..8).contains(&mouse_x) && (0..8).contains(&mouse_y) {
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                quad_ctx,
                DrawMode::stroke(tile_highlight_width),
                tile_rect(mouse_x, mouse_y),
                Color::WHITE,
            )?;
            graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;
        }

        if self.show_debug {
            draw_debug_text(
                ctx,
                quad_ctx,
                [
                    format!("Mouse down? {}", self.mouse.down),
                    format!("Mouse X: {}", self.mouse.x),
                    format!("Mouse Y: {}", self.mouse.y),
                ],
            )?;
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::GraphicsContext,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == MouseButton::Left {
            self.mouse.down = true;
        }
        self.mouse.x = x;
        self.mouse.y = y;
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::GraphicsContext,
        button: MouseButton,
        x: f32,
        y: f32,
    ) {
        if button == MouseButton::Left {
            self.mouse.down = false;
        }
        self.mouse.x = x;
        self.mouse.y = y;
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::GraphicsContext,
        x: f32,
        y: f32,
        _xrel: f32,
        _yrel: f32,
    ) {
        self.mouse.x = x;
        self.mouse.y = y;
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::GraphicsContext,
        keycode: KeyCode,
        keymod: KeyMods,
        _repeat: bool,
    ) {
        use KeyCode::*;

        match (keymod, keycode) {
            (KeyMods::NONE, F3) => self.show_debug ^= true,
            _ => (),
        }
    }
}
