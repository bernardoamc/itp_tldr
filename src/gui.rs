use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, Paragraph, Row, Table, Tabs},
};

use crate::database::{Domain, DomainInteraction};

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

    pub fn render_domain_list_widget(domains: &Vec<Domain>) -> List<'a> {
        let list_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Domains")
            .border_type(BorderType::Plain);

        let list_items: Vec<_> = domains
            .iter()
            .map(|domain| {
                ListItem::new(Spans::from(vec![Span::styled(
                    domain.name.clone(),
                    Style::default(),
                )]))
            })
            .collect();

        List::new(list_items).block(list_block).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        )
    }

    pub fn render_domain_info_widget(domain: &Domain) -> Table<'a> {
        let header_style = Style::default().add_modifier(Modifier::BOLD);

        Table::new(vec![Row::new(vec![
            Cell::from(Span::raw(domain.id.to_string())),
            Cell::from(Span::raw(domain.is_prevalent().to_owned())),
            Cell::from(Span::raw(domain.is_very_prevalent().to_owned())),
            Cell::from(Span::raw(domain.first_party_interaction.to_string())),
            Cell::from(Span::raw(domain.first_party_store_access.to_string())),
        ])])
        .header(Row::new(vec![
            Cell::from(Span::styled("ID", header_style)),
            Cell::from(Span::styled("PREVALENT?", header_style)),
            Cell::from(Span::styled("VERY PREVALENT?", header_style)),
            Cell::from(Span::styled("1ST PARTY USER INTERACTIONS", header_style)),
            Cell::from(Span::styled("ACCESS DUE TO STORAGE API", header_style)),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Details")
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
    }

    pub fn render_domain_interaction_widget(interaction: DomainInteraction) -> Table<'a> {
        let header_style = Style::default().add_modifier(Modifier::BOLD);

        Table::new(vec![Row::new(vec![
            Cell::from(Span::raw(interaction.iframes.to_string())),
            Cell::from(Span::raw(interaction.requests.to_string())),
            Cell::from(Span::raw(interaction.redirects.to_string())),
        ])])
        .header(Row::new(vec![
            Cell::from(Span::styled("IFRAMED", header_style)),
            Cell::from(Span::styled("CROSS ORIGIN REQUESTS TO", header_style)),
            Cell::from(Span::styled(
                "REDIRECTS WITHOUT USER INTERACTION",
                header_style,
            )),
        ]))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Interactions")
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(50),
        ])
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
