use gothic::renderable::{Canvas, Color, Image, TextMetrics};
use gothic::ui::geometry::{Point, Size};
use wasm4::{get_char_height, get_char_width};
use wasm4::framebuffer::{Framebuffer, PaletteIndex};

pub struct Wasm4Canvas<'a> {
    framebuffer: &'a Framebuffer,
}

impl<'a> Wasm4Canvas<'a> {
    pub fn new(framebuffer: &'a Framebuffer) -> Self {
        Self { framebuffer }
    }
}

impl<'a> Canvas for Wasm4Canvas<'a> {
    fn get_size(&self) -> Size {
        Size::new(self.framebuffer.get_screen_width(),
                  self.framebuffer.get_screen_height())
    }

    // Line

    fn set_line_color(&self, color: Color) {
        self.framebuffer.set_draw_colors([
            Some(color_to_palette_index(color)),
            None,
            None,
            None,
        ])
    }

    fn draw_line(&self, start: Point, end: Point) {
        self.framebuffer.line(start.x, start.y, end.x, end.y);
    }

    // Rectangle

    fn set_rectangle_color(&self, fill_color: Color, outline_color: Color) {
        self.framebuffer.set_draw_colors([
            Some(color_to_palette_index(fill_color)),
            Some(color_to_palette_index(outline_color)),
            None,
            None,
        ]);
    }

    fn draw_rectangle(&self, start: Point, size: Size) {
        self.framebuffer.rectangle(start.x, start.y, size.width, size.height);
    }

    // Text

    fn get_text_metrics(&self) -> TextMetrics {
        TextMetrics {
            line_height: get_char_height(),
            average_character_width: get_char_width(),
            maximum_character_width: get_char_width(),
        }
    }

    fn get_text_size(&self, text: &str) -> Size {
        let lines_widths = text.lines().map(|line| line.len());
        let max_width = lines_widths.clone().max().unwrap_or(0) as u32;
        let lines_count = lines_widths.count() as u32;
        let text_metrics = self.get_text_metrics();
        Size::new(
            max_width * text_metrics.maximum_character_width, // All system character are monospace
            lines_count * text_metrics.line_height,
        )
    }

    fn set_text_color(&self, foreground: Color, background: Color) {
        self.framebuffer.set_draw_colors([
            Some(color_to_palette_index(foreground)),
            Some(color_to_palette_index(background)),
            None,
            None,
        ]);
    }

    fn draw_text(&self, text: &str, start: Point) {
        self.framebuffer.text(text, start.x, start.y);
    }

    // Image

    fn set_image_colors(&self, colors: [Color; 4]) {
        self.framebuffer.set_draw_colors([
            Some(color_to_palette_index(colors[0])),
            Some(color_to_palette_index(colors[1])),
            Some(color_to_palette_index(colors[2])),
            Some(color_to_palette_index(colors[3])),
        ]);
    }

    fn draw_image(&self, image: &dyn Image, start: Point) {
        // let sprite = (image as &dyn Any).downcast_ref::<Sprite>().unwrap();
        // TODO
    }
}

fn color_to_palette_index(color: Color) -> PaletteIndex {
    match color {
        Color::Transparent => PaletteIndex::Transparent,
        Color::Background => PaletteIndex::Palette1,
        Color::Primary => PaletteIndex::Palette2,
        Color::Secondary => PaletteIndex::Palette3,
        Color::Tertiary => PaletteIndex::Palette4,
    }
}