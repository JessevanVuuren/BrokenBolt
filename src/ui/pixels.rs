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
    rect: Rect,
    bg: Color,
    fg: Color,
    pixels: Vec<Pixel>,
}

impl Pixels {
    pub fn new(rect: &Rect) -> Self {
        Self {
            rect: *rect,
            bg: Color::Black,
            fg: Color::White,
            pixels: Vec::new(),
        }
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

            buf[(p.x, p.y)].set_char(p.char).set_style(style);
        }
    }
}
