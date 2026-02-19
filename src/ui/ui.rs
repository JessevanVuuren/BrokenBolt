use core::num;
use std::{cmp, collections::BTreeMap, i32};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Widget, Wrap,
        canvas::{Canvas, Circle, Context, Map, MapResolution, Points, Rectangle},
    },
};

use crate::{
    handler::candle::Candle,
    types::types::CandleStick,
    ui::{
        app::App,
        pixels::{Pixel, Pixels},
        utils::{layout_block_f, layout_block_i},
    },
};

const BULL_COLOR: Color = Color::Rgb(52, 208, 88);
const BEAR_COLOR: Color = Color::Rgb(234, 74, 90);

pub fn ui(frame: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[0]);

    let top_right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(11), Constraint::Fill(1)])
        .split(top_layout[1]);

    let orderbook_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(top_right_layout[0]);

    let block = Block::new().borders(Borders::ALL).title("Bottom area");
    frame.render_widget(block, main_layout[1]);

    let block_i = Block::new().borders(Borders::ALL).title("Top left area");
    frame.render_widget(&block_i, top_layout[0]);

    let block = Block::new().borders(Borders::ALL).title("Top right bottom area");
    frame.render_widget(block, top_right_layout[1]);

    //
    //
    //

    //
    // order book
    //
    let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];

    let book = &app.orderbook;

    let ask_scale: i64 = book.asks.iter().map(|t| t.1.clone()).sum();
    let bid_scale: i64 = book.bids.iter().map(|t| t.1.clone()).sum();
    let max_scale = cmp::max(ask_scale, bid_scale);

    let mut scale_ask = 0.;
    let ask_rows = book.asks.iter().map(|t| {
        let (price, qty) = book.ask_decode(t);

        let width = orderbook_layout[0].width as f64;
        scale_ask += t.1.clone() as f64 / max_scale as f64;
        let bar_width = (1. - scale_ask) * width;

        order_book_row(qty, price, width, bar_width, BEAR_COLOR, Color::Black)
    });

    let mut scale_bid = 0.;
    let bid_rows = book.bids.iter().map(|t| {
        let (price, qty) = book.bid_decode(t);

        let width = orderbook_layout[1].width as f64;
        scale_bid += t.1.clone() as f64 / max_scale as f64;
        let bar_width = scale_bid * width;

        order_book_row(price, qty, width, bar_width, Color::Black, BULL_COLOR)
    });

    let ask_table = Table::new(ask_rows, widths).header(Row::new(vec!["quantity", "price"]));
    let bid_table = Table::new(bid_rows, widths).header(Row::new(vec!["price", "quantity"]));

    frame.render_widget(ask_table, orderbook_layout[0]);
    frame.render_widget(bid_table, orderbook_layout[1]);

    //
    //
    //

    //
    // candle sticks
    //

    let mut pixels = Pixels::new(&block_i.inner(top_layout[0]));
    let candle_range = app.candle.min_max(pixels.width() as usize);
    let mut candle_pixels = build_candle_pixels(&pixels.rect, &app.candle.candles, candle_range, BULL_COLOR, BEAR_COLOR);

    pixels.flip_y = true;
    pixels.flip_x = true;

    pixels.add_pixels(&mut candle_pixels);
    frame.render_widget(pixels, top_layout[0]);
}

fn build_candle_pixels(rect: &Rect, candles: &Vec<CandleStick>, range: (f64, f64), bull: Color, bear: Color) -> Vec<Pixel> {
    let (x, y, w, h) = layout_block_f(rect);

    let scaler = range.1 - range.0;

    let mut vec = Vec::new();
    for (index_x, candle) in candles.iter().enumerate().take(w as usize) {
        let open = (candle.open - range.0) / scaler * h;
        let high = (candle.high - range.0) / scaler * h;
        let low = (candle.low - range.0) / scaler * h;
        let close = (candle.close - range.0) / scaler * h;

        let start_cell = low.floor() as u64;
        let stop_cell = high.floor() as u64;

        let diff = start_cell.abs_diff(stop_cell + 1);
        let color = if candle.open < candle.close { bull } else { bear };

        for i in 0..diff {
            let pos_x = index_x + x as usize;
            let pos_y = (start_cell + i) as f64;

            let o = open - pos_y;
            let h = high - pos_y;
            let l = low - pos_y;
            let c = close - pos_y;

            if let Some(ch) = candle_stick(o, h, l, c) {
                let pixel = Pixel::new(pos_x as u16, pos_y as u16 + y as u16).char(ch).fg(color);
                vec.push(pixel);
            }
        }
    }

    vec
}

fn candle_stick(open: f64, high: f64, low: f64, close: f64) -> Option<char> {
    let bar_low = f64::min(open, close);
    let bar_high = f64::max(open, close);

    let wick_low = f64::min(high, low);
    let wick_high = f64::max(high, low);

    if wick_high > 0.75 && wick_low < 0.25 && bar_high > 0.75 && bar_low < 0.25 {
        return Some('┃');
    }

    if wick_high > 0.75 && wick_low < 0.25 && (bar_low > 0.75 || bar_high < 0.25) {
        return Some('│');
    }

    if high < 0.75 && high > 0.25 && wick_low < 0.25 && bar_low < 0.25 {
        return Some('╷');
    }

    if low < 0.75 && low > 0.25 && wick_low > 0.75 && bar_low > 0.75 {
        return Some('╵');
    }

    if bar_high > 0.75 && bar_low > 0.25 && wick_low < 0.25 {
        return Some('╿');
    }

    if bar_high < 0.75 && bar_low < 0.25 && wick_high > 0.75 {
        return Some('╽');
    }

    if bar_high < 0.75 && bar_low > 0.25 {
        return Some('╻');
    }

    if bar_high > 0.75 && bar_low < 0.25 {
        return Some('╹');
    }

    None
}

fn order_book_row(val1: f64, val2: f64, width: f64, bar_width: f64, color1: Color, color2: Color) -> Row<'static> {
    let half = width / 2.;
    let style1 = Style::new().bg(color1);
    let style2 = Style::new().bg(color2);

    let mut strs_val1: Vec<Span<'static>> = Vec::new();
    let mut strs_val2: Vec<Span<'static>> = Vec::new();

    let padding = " ".repeat(width as usize);
    let val1 = format!("{val1}{padding}");
    let val2 = format!("{val2}{padding}");

    if let Some((left, right)) = val1.split_at_checked(bar_width as usize) {
        strs_val1.push(Span::from(left.to_owned()).style(style2));
        strs_val1.push(Span::from(right.to_owned()).style(style1));
    }

    if let Some((left, right)) = val2.split_at_checked((bar_width - half - 1.0) as usize) {
        strs_val2.push(Span::from(left.to_owned()).style(style2));
        strs_val2.push(Span::from(right.to_owned()).style(style1));
    } else {
        strs_val2.push(Span::from(val2).style(style1));
    }

    let style = if bar_width > half { style2 } else { style1 };

    Row::new(vec![Cell::from(Line::from(strs_val1)), Cell::from(Line::from(strs_val2))]).style(style)
}
