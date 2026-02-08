use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::Widget,
};

#[derive(Debug, Clone)]
pub struct Pixel {
    x: u16,
    y: u16,
    char: char,
    bg: Option<Color>,
    fg: Option<Color>,
}

impl Pixel {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
            char: ' ',
            bg: None,
            fg: None,
        }
    }

    pub fn bg(mut self, c: Color) -> Self {
        self.bg = Some(c);
        self
    }

    pub fn fg(mut self, c: Color) -> Self {
        self.fg = Some(c);
        self
    }

    pub fn char(mut self, c: char) -> Self {
        self.char = c;
        self
    }
}

#[derive(Debug, Clone)]
pub struct Pixels {
    pub rect: Rect,
    bg: Color,
    fg: Color,
    pixels: Vec<Pixel>,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Pixels {
    pub fn new(rect: &Rect) -> Self {
        Self {
            rect: Rect {
                x: rect.x,
                y: rect.y,
                width: rect.width,
                height: rect.height,
            },
            bg: Color::Black,
            fg: Color::White,
            pixels: Vec::new(),
            flip_x: false,
            flip_y: false,
        }
    }

    pub fn height(&self) -> u16 {
        self.rect.height
    }

    pub fn width(&self) -> u16 {
        self.rect.width
    }

    pub fn bg(mut self, c: Color) -> Self {
        self.bg = c;
        self
    }

    pub fn fg(mut self, c: Color) -> Self {
        self.fg = c;
        self
    }

    pub fn add_pixel(&mut self, p: Pixel) {
        self.pixels.push(p);
    }

    pub fn add_pixels(&mut self, ps: &mut Vec<Pixel>) {
        self.pixels.append(ps);
    }
}

impl Widget for Pixels {
    fn render(self, area: Rect, buf: &mut Buffer) {
        buf.set_style(area, Style::new().bg(self.bg));

        for p in &self.pixels {
            let style = Style::new().bg(p.bg.unwrap_or(self.bg)).fg(p.fg.unwrap_or(self.fg));

            let mut pos = (p.x, p.y);

            if (self.flip_x) {
                pos.0 = self.rect.width - pos.0 + 1;
            }

            if (self.flip_y) {
                pos.1 = self.rect.height - pos.1 + 1;
            }

            buf[pos].set_char(p.char).set_style(style);
        }
    }
}
