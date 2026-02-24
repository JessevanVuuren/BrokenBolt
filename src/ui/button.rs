use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style},
    widgets::Widget,
};

use crate::ui::utils::rgb_brightness;

type Callback = Box<dyn Fn() + Send + 'static>;

pub struct Button {
    pub text: String,
    pub rect: Rect,

    callback: Callback,
    highlighted: bool,

    bg: Color,
    fg: Color,
}

fn holder() {}

impl Button {
    pub fn new(width: u16, height: u16, text: &str, rect: &Rect) -> Self {
        Self {
            fg: Color::Rgb(255, 255, 255),
            bg: Color::Rgb(50, 50, 50),
            text: text.to_string(),
            highlighted: false,
            callback: Box::new(holder),
            rect: Rect {
                x: rect.x,
                y: rect.y,
                width,
                height,
            },
        }
    }
    pub fn callback<F: Fn() + Send + 'static>(&mut self, cb: F) {
        self.callback = Box::new(cb);
    }

    pub fn mouse(&mut self, mouse: &Option<MouseEvent>) {
        self.highlighted = false;

        if let Some(mouse) = mouse {
            let position = Position {
                x: mouse.column,
                y: mouse.row,
            };
            if self.rect.contains((position)) {
                self.highlighted = true;

                if let MouseEventKind::Down(_) = mouse.kind {
                    (self.callback)();
                }
            }
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
}

impl Widget for &Button {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let high = rgb_brightness(self.bg, 25);
        let low = rgb_brightness(self.bg, -25);

        let text_width = self.text.chars().count() as u16;

        let bg = if self.highlighted { rgb_brightness(self.bg, 20) } else { self.bg };

        buf.set_style(self.rect, Style::new().bg(bg));

        if self.rect.height > 3 {
            let text = "▁".repeat(self.rect.width as usize);
            buf.set_string(area.x, area.y + self.rect.height - 1, text, Style::new().fg(low));
        }

        if self.rect.height > 2 {
            buf.set_string(area.x, area.y, "▔".repeat(self.rect.width as usize), Style::new().fg(high));
        }

        let y = area.y + self.rect.height / 2;
        let x = area.x + area.width.saturating_sub(text_width) / 2;

        if self.rect.height == 1 && self.rect.width > text_width + 2 {
            buf.set_string(area.x, y, "[", Style::new().bold().fg(high));
            buf.set_string(area.x + self.rect.width - 1, y, "]", Style::new().bold().fg(low));
        }
        buf.set_string(x, y, self.text.clone(), Style::new().bold().fg(self.fg));
    }
}
