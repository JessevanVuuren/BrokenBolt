use ratatui::layout::Rect;

pub fn layout_block_f(rect: &Rect) -> (f64, f64, f64, f64) {
    (rect.x as f64, rect.y as f64, rect.width as f64, rect.height as f64)
}

pub fn layout_block_i(rect: &Rect) -> (i64, i64, i64, i64) {
    (rect.x as i64, rect.y as i64, rect.width as i64, rect.height as i64)
}

pub fn layout_block_u(rect: &Rect) -> (u16, u16, u16, u16) {
    (rect.x, rect.y, rect.width, rect.height)
}
