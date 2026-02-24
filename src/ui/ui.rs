use core::num;
use std::{cmp, collections::BTreeMap, fmt::format, i32, sync::mpsc::Sender};

use crossterm::event::MouseEvent;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect, Rows},
    style::{Color, Style, Stylize},
    symbols::Marker,
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Widget, Wrap,
        canvas::{Canvas, Circle, Context, Map, MapResolution, Points, Rectangle},
    },
};
use tokio::fs::try_exists;

use crate::{
    Message, epoch_to_rfc3339, epoch_to_timestamp,
    fetch::types::Trade,
    handler::{candle::Candle, trades},
    types::types::CandleStick,
    ui::{
        app::App,
        button::Button,
        globals::{BEAR_COLOR, BULL_COLOR},
        pixels::{Pixel, Pixels},
        ui_buttons::ui_buttons,
        utils::{abs_scale_rect, layout_block_f, layout_block_i, offset_rect, scale_rect},
    },
};

pub fn ui(mut frame: &mut Frame, app: &App, mouse: &Option<MouseEvent>, update: Sender<Message>) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(frame.area());

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(main_layout[0]);

    let top_right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(13), Constraint::Fill(1)])
        .split(top_layout[1]);

    let orderbook_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(top_right_layout[0]);

    let block_trades = Block::new().borders(Borders::ALL).title(" All executed trades ");
    frame.render_widget(&block_trades, main_layout[1]);

    let title = format!(" Candle stick chart: {}, interval: {}m ", app.candle.pair, app.candle.interval);
    let block_candle = Block::new().borders(Borders::ALL).title(title);
    frame.render_widget(&block_candle, top_layout[0]);

    let block_orderbook = Block::new().borders(Borders::ALL).title(" Orderbook ");
    frame.render_widget(&block_orderbook, top_right_layout[0]);

    //
    //
    //

    //
    // order book
    //
    let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];

    let book = &app.orderbook;
    let book_ask_area = &block_orderbook.inner(orderbook_layout[0]);
    let book_bid_area = &block_orderbook.inner(orderbook_layout[1]);

    let ask_scale: i64 = book.asks.iter().map(|t| t.1.clone()).sum();
    let bid_scale: i64 = book.bids.iter().map(|t| t.1.clone()).sum();
    let max_scale = cmp::max(ask_scale, bid_scale);

    let mut scale_ask = 0.;
    let ask_rows = book.asks.iter().map(|t| {
        let (price, qty) = book.ask_decode(t);

        let width = book_ask_area.width as f64;
        scale_ask += t.1.clone() as f64 / max_scale as f64;
        let bar_width = (1. - scale_ask) * width;

        order_book_row(qty, price, width, bar_width, BEAR_COLOR, Color::Black)
    });

    let mut scale_bid = 0.;
    let bid_rows = book.bids.iter().map(|t| {
        let (price, qty) = book.bid_decode(t);

        let width = book_bid_area.width as f64;
        scale_bid += t.1.clone() as f64 / max_scale as f64;
        let bar_width = scale_bid * width;

        order_book_row(price, qty, width, bar_width, Color::Black, BULL_COLOR)
    });

    let header_style = Style::new().bg(Color::Rgb(50, 50, 50)).bold();
    let ask_table = Table::new(ask_rows, widths).header(Row::new(vec!["QUANTITY", "PRICE"]).style(header_style));
    let bid_table = Table::new(bid_rows, widths).header(Row::new(vec!["PRICE", "QUANTITY"]).style(header_style));

    frame.render_widget(ask_table, *book_ask_area);
    frame.render_widget(bid_table, *book_bid_area);

    //
    //
    //

    //
    // candle sticks
    //

    let mut pixels = Pixels::new(&block_candle.inner(top_layout[0]));
    let candle_range = app.candle.min_max(pixels.width() as usize);
    let mut candle_pixels = build_candle_pixels(&pixels.rect, &app.candle.candles, candle_range, BULL_COLOR, BEAR_COLOR);

    pixels.flip_y = true;
    pixels.flip_x = true;

    pixels.add_pixels(&mut candle_pixels);
    frame.render_widget(pixels, top_layout[0]);

    //
    //
    //

    //
    // trade history
    //

    let headers = ["TIME", "PAIR", "TYPE", "VOLUME", "COST"];
    let header_style = Style::new().bg(Color::Rgb(50, 50, 50)).bold();
    let widths = [Constraint::Percentage((100 / headers.len()) as u16)].repeat(headers.len());

    let trade_rows = trades_table_rows(&widths, &headers, &app.trades.trades);
    let trades_table = Table::new(trade_rows, widths).header(Row::new(headers).style(header_style));

    frame.render_widget(trades_table, block_trades.inner(main_layout[1]));

    //
    //
    //

    //
    // buttons
    //
    ui_buttons(top_right_layout[1], &mut frame, mouse, update);
}

fn trades_table_rows(widths: &[Constraint], headers: &[&str], trades: &[Trade]) -> Vec<Row<'static>> {
    trades
        .iter()
        .enumerate()
        .map(|(i, trade)| {
            let style = if i % 2 == 1 {
                Style::new().bg(Color::Rgb(30, 30, 30))
            } else {
                Style::new().bg(Color::Black)
            };

            Row::new(vec![
                Cell::from(epoch_to_timestamp(trade.time as u64)),
                Cell::from(trade.pair.clone()),
                Cell::from(trade.type_field.clone()),
                Cell::from(trade.vol.to_string()),
                Cell::from(trade.cost.to_string()),
            ])
            .style(style)
        })
        .collect()
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
