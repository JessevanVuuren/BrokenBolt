use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
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
        .constraints([Constraint::Max(10), Constraint::Fill(1)])
        .split(top_layout[1]);

    let orderbook_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(top_right_layout[0]);

    let block = Block::new().borders(Borders::ALL).title("Bottom area");
    frame.render_widget(block, main_layout[1]);

    let block = Block::new().borders(Borders::ALL).title("Top left area");
    frame.render_widget(block, top_layout[0]);

    let block = Block::new()
        .borders(Borders::ALL)
        .title("Top right bottom area");
    frame.render_widget(block, top_right_layout[1]);

    let widths = [Constraint::Percentage(50), Constraint::Percentage(50)];

    let ask_rows = app.orderbook.asks.iter().enumerate().map(|(i, t)| {
        let color = match i % 2 {
            0 => Color::Blue,
            _ => Color::Red,
        };
        Row::new(vec![
            Cell::from(t.0.to_string()),
            Cell::from(t.1.to_string()),
        ])
        .style(Style::new().bg(color))
    });

    let bid_rows = app.orderbook.bids.iter().enumerate().map(|(i, t)| {
        let color = match i % 2 {
            0 => Color::Blue,
            _ => Color::Red,
        };
        Row::new(vec![
            Cell::from(t.0.0.to_string()),
            Cell::from(t.1.to_string()),
        ])
        .style(Style::new().bg(color))
    });

    let ask_table = Table::new(ask_rows, widths).header(Row::new(vec!["price", "quantity"]));
    let bid_table = Table::new(bid_rows, widths).header(Row::new(vec!["price", "quantity"]));

    frame.render_widget(ask_table, orderbook_layout[0]);
    frame.render_widget(bid_table, orderbook_layout[1]);
}
