use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
};
pub struct Gui;

impl<'a> Gui {
    pub fn render_panes(screen_size: Rect) -> Vec<Rect> {
        Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(2),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(screen_size)
    }

    pub fn render_tabs(titles: &'a Vec<&'a str>, selected_item_index: usize) -> Tabs {
        let menu = titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(Gui::render_menu_style(first, rest))
            })
            .collect();

        Tabs::new(menu)
            .select(selected_item_index)
            .block(Block::default().title("Menu").borders(Borders::ALL))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().fg(Color::Yellow))
            .divider(Span::raw("|"))
    }

    pub fn render_copyright() -> Paragraph<'a> {
        Paragraph::new("ITP TL;DR; 2021")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .title("Copyright")
                    .border_type(BorderType::Plain),
            )
    }

    pub fn render_home_pane() -> Paragraph<'a> {
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Welcome")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("to")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "ITP TL;DR;",
                Style::default().fg(Color::LightBlue),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw(
                "Press 'h' for Home, 'd' for Domains or 'q' to quit.",
            )]),
            Spans::from(vec![Span::raw("Navigate through domains with arrow keys.")]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Home")
                .border_type(BorderType::Plain),
        );
        home
    }

    fn render_menu_style(first: &'a str, rest: &'a str) -> Vec<Span<'a>> {
        vec![
            Span::styled(
                first,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::UNDERLINED),
            ),
            Span::styled(rest, Style::default().fg(Color::White)),
        ]
    }
}
