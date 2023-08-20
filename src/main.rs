extern crate good_web_game as ggez;

mod board;
mod debug;

use ggez::cgmath::Point2;
use ggez::event::EventHandler;
use ggez::event::KeyMods;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::DrawParam;
use ggez::input::MouseButton;
use ggez::miniquad;
use ggez::miniquad::KeyCode;
use ggez::Context;
use ggez::GameResult;

use crate::board::Board;
use crate::debug::draw_debug_text;

fn main() -> GameResult<()> {
    ggez::start(ggez::conf::Conf::default(), |mut context, quad_ctx| {
        Box::new(Main::new(&mut context, quad_ctx).unwrap())
    })
}

#[macro_export]
macro_rules! color {
    ($name:ident) => {
        ::ggez::graphics::Color::$name
    };
    ($r:expr, $g:expr, $b:expr) => {
        ::ggez::graphics::Color::new(
            $r as u8 as f32 / 255.0,
            $g as u8 as f32 / 255.0,
            $b as u8 as f32 / 255.0,
            255.0,
        )
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        ::ggez::graphics::Color::new(
            $r as u8 as f32 / 255.0,
            $g as u8 as f32 / 255.0,
            $b as u8 as f32 / 255.0,
            $a as u8 as f32 / 255.0,
        )
    };
}

const BOARD_MARGIN: f32 = 20.0;
const TILE_SIZE: f32 = 60.0;
const TILE_HOVER_STROKE: f32 = 4.0;
const TILE_ACTIVE_STROKE: f32 = 2.0;
const COLOR_TILE_A: Color = color!(140.0, 80.0, 50.0, 255.0);
const COLOR_TILE_B: Color = color!(80, 30, 20);
const COLOR_ACTIVE: Color = color!(255, 255, 255);

#[derive(Default)]
struct Main {
    frame_count: u32,
    mouse: MouseState,
    show_debug: bool,
    tile_active: Option<(i32, i32)>,
    tile_hover: Option<(i32, i32)>,
    board: Board,
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
            ..Default::default()
        };

        Ok(state)
    }
}

impl EventHandler for Main {
    fn update(
        &mut self,
        _ctx: &mut Context,
        _quad_ctx: &mut miniquad::GraphicsContext,
    ) -> GameResult {
        self.frame_count += 1;

        let mouse_x = ((self.mouse.x - BOARD_MARGIN) / TILE_SIZE) as i32;
        let mouse_y = ((self.mouse.y - BOARD_MARGIN) / TILE_SIZE) as i32;

        self.tile_hover = if (0..8).contains(&mouse_x) && (0..8).contains(&mouse_y) {
            Some((mouse_x, mouse_y))
        } else {
            None
        };

        if self.mouse.down {
            self.tile_active = self.tile_hover;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, quad_ctx: &mut miniquad::GraphicsContext) -> GameResult {
        graphics::clear(ctx, quad_ctx, color!(BLACK));

        let tile_rect = |x: i32, y: i32| -> graphics::Rect {
            graphics::Rect::new(
                x as f32 * TILE_SIZE + BOARD_MARGIN,
                y as f32 * TILE_SIZE + BOARD_MARGIN,
                TILE_SIZE,
                TILE_SIZE,
            )
        };

        let tile_color = |x: i32, y: i32| -> Color {
            if (x + y) % 2 == 0 {
                COLOR_TILE_A
            } else {
                COLOR_TILE_B
            }
        };

        for y in 0..8 {
            for x in 0..8 {
                let rect = tile_rect(x, y);

                let mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    quad_ctx,
                    DrawMode::fill(),
                    rect,
                    tile_color(x, y),
                )?;
                graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;

                if let Some(piece) = self.board.tile_at_coords(x, y) {
                    let text = piece.symbol();

                    let font_size = 70.0;
                    let text_color = color!(WHITE);

                    let position = Point2::new(rect.x + TILE_SIZE / 2.0, rect.y);

                    let mut text =
                        graphics::Text::new((text, graphics::Font::default(), font_size));

                    text.set_bounds(Point2::new(1.0, f32::INFINITY), graphics::Align::Center);

                    graphics::draw(ctx, quad_ctx, &text, (position, 0.0, text_color))?;
                }
            }
        }

        if let Some((x, y)) = self.tile_hover {
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                quad_ctx,
                DrawMode::stroke(TILE_HOVER_STROKE),
                tile_rect(x, y),
                tile_color(x, y),
            )?;
            graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;
        }

        if let Some((x, y)) = self.tile_active {
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                quad_ctx,
                DrawMode::stroke(TILE_ACTIVE_STROKE),
                tile_rect(x, y),
                COLOR_ACTIVE,
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
                    format!("Total frames: {}", self.frame_count),
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
