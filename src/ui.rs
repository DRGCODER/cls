use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(3), 
        Constraint::Min(10),   
        Constraint::Length(3), 
    ])
    .split(frame.area());

    render_header(frame, chunks[0]);
    render_stats_table(frame, chunks[1], app);
    render_footer(frame, chunks[2], app);
}

fn render_header(frame: &mut Frame, area: Rect) {
    let header = Paragraph::new("Code Line Statistics")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, area);
}

fn render_stats_table(frame: &mut Frame, area: Rect, app: &App) {
    let header = Row::new(vec!["Language", "Code", "Comments", "Blanks", "Files", "Total"])
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .bottom_margin(1);

    let rows: Vec<Row> = app
        .stats()
        .languages()
        .iter()
        .map(|lang| {
            Row::new(vec![
                lang.name.to_string(),
                lang.code.to_string(),
                lang.comments.to_string(),
                lang.blanks.to_string(),
                lang.files.to_string(),
                lang.total_lines().to_string(),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(15),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(Block::default().borders(Borders::ALL).title("Statistics"));

    frame.render_widget(table, area);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let stats = app.stats();
    let footer_text = format!(
        "Total: {} code | {} comments | {} blanks | {} files | Press 'q' to quit",
        stats.total_code(),
        stats.total_comments(),
        stats.total_blanks(),
        stats.total_files(),
    );

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(footer, area);
}

