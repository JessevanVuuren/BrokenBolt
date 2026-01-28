use std::{cmp, collections::BTreeMap};

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, List, ListItem, Paragraph, Row, Table, Wrap},
};

use crate::ui::app::App;

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

    let block = Block::new().borders(Borders::ALL).title("Top left area");
    frame.render_widget(block, top_layout[0]);

    let block = Block::new().borders(Borders::ALL).title("Top right bottom area");
    frame.render_widget(block, top_right_layout[1]);

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

        order_book_row(qty, price, width, bar_width, Color::Rgb(180, 15, 15), Color::Black)
    });

    let mut scale_bid = 0.;
    let bid_rows = book.bids.iter().map(|t| {
        let (price, qty) = book.bid_decode(t);

        let width = orderbook_layout[1].width as f64;
        scale_bid += t.1.clone() as f64 / max_scale as f64;
        let bar_width = scale_bid * width;

        order_book_row(price, qty, width, bar_width, Color::Black, Color::Rgb(0, 120, 0))
    });

    let ask_table = Table::new(ask_rows, widths).header(Row::new(vec!["quantity", "price"]));
    let bid_table = Table::new(bid_rows, widths).header(Row::new(vec!["price", "quantity"]));

    frame.render_widget(ask_table, orderbook_layout[0]);
    frame.render_widget(bid_table, orderbook_layout[1]);
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
