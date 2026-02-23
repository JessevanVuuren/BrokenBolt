use ratatui::{layout::Rect, style::Color};

pub fn layout_block_f(rect: &Rect) -> (f64, f64, f64, f64) {
    (rect.x as f64, rect.y as f64, rect.width as f64, rect.height as f64)
}

pub fn layout_block_i(rect: &Rect) -> (i64, i64, i64, i64) {
    (rect.x as i64, rect.y as i64, rect.width as i64, rect.height as i64)
}

pub fn layout_block_u(rect: &Rect) -> (u16, u16, u16, u16) {
    (rect.x, rect.y, rect.width, rect.height)
}

pub fn offset_rect(rect: &Rect, x: u16, y: u16) -> Rect {
    Rect {
        x: rect.x + x,
        y: rect.y + y,
        height: rect.height,
        width: rect.width,
    }
}

pub fn rgb_brightness(color: Color, brightness: i16) -> Color {
    match color {
        Color::Rgb(r, g, b) => {
            let r = (r as i16 + brightness).clamp(0, 255) as u8;
            let g = (g as i16 + brightness).clamp(0, 255) as u8;
            let b = (b as i16 + brightness).clamp(0, 255) as u8;
            Color::Rgb(r, g, b)
        }
        other => other,
    }
}
