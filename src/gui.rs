use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table, Tabs,
    },
};

use crate::database::Database;

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

    pub fn render_domains(
        database: &Database,
        domain_list_state: &ListState,
    ) -> (List<'a>, Table<'a>) {
        let domains_pane = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Domains")
            .border_type(BorderType::Plain);

        let domain_list = database.get_domains().expect("fetch domain list");
        let items: Vec<_> = domain_list
            .iter()
            .map(|domain| {
                ListItem::new(Spans::from(vec![Span::styled(
                    domain.name.clone(),
                    Style::default(),
                )]))
            })
            .collect();

        let domains_list_ui = List::new(items).block(domains_pane).highlight_style(
            Style::default()
                .bg(Color::Yellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD),
        );

        let selected_domain = domain_list
            .get(domain_list_state.selected().expect("domain to be selected"))
            .expect("domain exists");

        let domain_info = database
            .get_info(&selected_domain)
            .expect("get information from domain");

        let header_style = Style::default().add_modifier(Modifier::BOLD);
        let domain_info_pane = Table::new(vec![Row::new(vec![
            Cell::from(Span::raw(domain_info.id.to_string())),
            Cell::from(Span::raw(domain_info.render_is_prevalent().to_owned())),
            Cell::from(Span::raw(domain_info.render_is_very_prevalent().to_owned())),
            Cell::from(Span::raw(domain_info.first_party_interaction.to_string())),
            Cell::from(Span::raw(domain_info.first_party_store_access.to_string())),
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
                .title("Detail")
                .border_type(BorderType::Plain),
        )
        .widths(&[
            Constraint::Percentage(10),
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ]);

        (domains_list_ui, domain_info_pane)
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
