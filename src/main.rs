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
use ggez::cgmath::Vector2;
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
const TILE_HOVER_STROKE: f32 = 6.0;
const TILE_ACTIVE_STROKE: f32 = 5.0;
const COLOR_TILE_A: Color = color!(140, 80, 50);
const COLOR_TILE_B: Color = color!(80, 30, 20);
const COLOR_ACTIVE: Color = color!(220, 150, 120);

#[derive(Default)]
struct Main {
    board: Board,
    tile_active: Option<Coords>,
    tile_hover: Option<Coords>,
    previous_move: Option<(Coords, Coords)>,
    mouse: MouseState,
    events: Events,
    show_debug: bool,
    frame_count: u32,
}

//TODO: Change to u32
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}

impl Coords {
    pub fn new(x: i32, y: i32) -> Self {
        Coords { x, y }
    }

    pub fn as_point2(self) -> Point2<f32> {
        Point2 {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

#[derive(Default)]
struct Events {
    mouse_clicked: bool,
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

impl Main {
    fn move_piece(&mut self) {
        // A tile is currently active, and another is hovered
        let Some((old_coords, new_coords)) = self.tile_active.zip(self.tile_hover ) else {
            return;
        };
        // Check if tile is same position
        if old_coords == new_coords {
            return;
        }

        // Get value of old piece
        let Some(old) = self.board.tile_at_coords(old_coords) else { return };
        // Get mutable reference to new piece
        // `new` is Some if a piece is taken, or None if a move to an empty space
        let Some(new) = self.board.tile_at_coords_mut(new_coords) else { return };

        // Move piece by copying
        *new = old;
        // Delete old piece
        if let Some(old) = self.board.tile_at_coords_mut(old_coords) {
            *old = None;
        }

        self.previous_move = Some((old_coords, new_coords));
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
            Some(Coords::new(mouse_x, mouse_y))
        } else {
            None
        };

        if self.mouse.down {
            if !self.events.mouse_clicked {
                self.events.mouse_clicked = true;
                if self.tile_active.is_none() {
                    self.tile_active = self.tile_hover;
                } else {
                    self.move_piece();
                    self.tile_active = None;
                }
            }
        } else {
            self.events.mouse_clicked = false;
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

                if let Some(Some(piece)) = self.board.tile_at_coords(Coords { x, y }) {
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

        if let Some(Coords { x, y }) = self.tile_hover {
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                quad_ctx,
                DrawMode::stroke(TILE_HOVER_STROKE),
                tile_rect(x, y),
                tile_color(x, y),
            )?;
            graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;
        }

        if let Some(Coords { x, y }) = self.tile_active {
            let mesh = graphics::Mesh::new_rectangle(
                ctx,
                quad_ctx,
                DrawMode::stroke(TILE_ACTIVE_STROKE),
                tile_rect(x, y),
                COLOR_ACTIVE,
            )?;
            graphics::draw(ctx, quad_ctx, &mesh, DrawParam::default())?;
        }

        if let Some((old, new)) = self.previous_move {
            const MOVE_ARROW_WIDTH: f32 = 8.0;
            const COLOR_MOVE_ARROW: Color = color!(255, 255, 255, 100);

            let offset = Vector2 {
                x: BOARD_MARGIN + TILE_SIZE / 2.0,
                y: BOARD_MARGIN + TILE_SIZE / 2.0,
            };

            let points = &[
                old.as_point2() * TILE_SIZE + offset,
                new.as_point2() * TILE_SIZE + offset,
            ];

            let line = graphics::Mesh::new_line(
                ctx,
                quad_ctx,
                points,
                MOVE_ARROW_WIDTH,
                COLOR_MOVE_ARROW,
            )?;

            graphics::draw(ctx, quad_ctx, &line, DrawParam::default())?;
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
                    format!("Active tile: {:?}", self.tile_active),
                    format!("Hovered tile: {:?}", self.tile_active),
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
