use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Position, Rect},
    style::{Color, Style},
    widgets::Widget,
};

use crate::ui::utils::rgb_brightness;

type Callback = fn();

#[derive(Debug)]
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
    pub fn new(width: u16, height: u16, text: &str, rect: &Rect, cb: Callback) -> Self {
        Self {
            fg: Color::Rgb(255, 255, 255),
            bg: Color::Rgb(50, 50, 50),
            text: text.to_string(),
            highlighted: false,
            callback: cb,
            rect: Rect {
                x: rect.x,
                y: rect.y,
                width,
                height,
            },
        }
    }

    pub fn callback(&mut self, cb: Callback) {
        self.callback = cb
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

        let bg = if self.highlighted { rgb_brightness(self.bg, 20) } else { self.bg };

        buf.set_style(self.rect, Style::new().bg(bg));

        buf.set_string(
            area.x,
            area.y + self.rect.height - 1,
            "▁".repeat(self.rect.width as usize),
            Style::new().fg(low),
        );
        buf.set_string(area.x, area.y, "▔".repeat(self.rect.width as usize), Style::new().fg(high));

        let y = area.y + self.rect.height / 2;
        let x = area.x + self.rect.width / 2 - (self.text.chars().count() as u16) / 2;

        buf.set_string(x, y, self.text.to_string(), Style::new().bold().fg(self.fg));
    }
}
