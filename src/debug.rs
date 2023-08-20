use ggez::graphics::TextFragment;
use ggez::cgmath::Point2;
use ggez::graphics;
use ggez::graphics::DrawMode;
use ggez::graphics::DrawParam;
use ggez::miniquad;
use ggez::Context;
use ggez::GameResult;

use crate::color;

pub fn draw_debug_text<const N: usize>(
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
