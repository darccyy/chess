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

macro_rules! color {
    ($name:ident) => {
        Color::$name
    };
    ($r:expr, $g:expr, $b:expr) => {
        Color::new(
            $r as u8 as f32 / 255.0,
            $g as u8 as f32 / 255.0,
            $b as u8 as f32 / 255.0,
            255.0,
        )
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color::new(
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

struct Board(Grid);

type Grid = [[TileState; 8]; 8];

macro_rules! piece {
    ($color:ident $kind:ident) => {
        Piece {
            kind: PieceKind::$kind,
            color: PieceColor::$color,
        }
    };
}

impl Default for Board {
    fn default() -> Self {
        let mut grid: Grid = Default::default();

        for x in 0..8 {
            grid[1][x] = Some(piece!(White Pawn));
        }
        for x in 0..8 {
            grid[6][x] = Some(piece!(Black Pawn));
        }

        grid[0][1] = Some(piece!(White Rook));
        grid[0][6] = Some(piece!(White Rook));
        grid[7][1] = Some(piece!(Black Rook));
        grid[7][6] = Some(piece!(Black Rook));

        Board(grid)
    }
}

impl Board {
    pub fn tile_at_coords(&self, x: i32, y: i32) -> TileState {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;
        *self.0.get(y)?.get(x)?
    }
}

#[derive(Default)]
struct MouseState {
    down: bool,
    x: f32,
    y: f32,
}

type TileState = Option<Piece>;

#[derive(Clone, Copy)]
struct Piece {
    kind: PieceKind,
    color: PieceColor,
}
#[derive(Clone, Copy)]
enum PieceKind {
    Pawn,
    Rook,
}
#[derive(Clone, Copy, PartialEq)]
enum PieceColor {
    White,
    Black,
}

impl Piece {
    fn symbol(&self) -> &'static str {
        let (white, black) = match self.kind {
            PieceKind::Pawn => ("p", "P"),
            PieceKind::Rook => ("r", "R"),
            // Self::Pawn => ("♙", "♟"),
            // Self::Rook => ("♖", "♜"),
        };

        if self.color == PieceColor::White {
            white
        } else {
            black
        }
    }
}

impl Main {
    pub fn new(_ctx: &mut Context, _quad_ctx: &mut miniquad::GraphicsContext) -> GameResult<Self> {
        let state = Main {
            ..Default::default()
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

    let text_color = color!(WHITE);
    let background_color = color!(128, 0, 0, 128);

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
