use std::io::Stdout;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Cell, List, ListItem, ListState, Paragraph, Row, Table},
    Frame,
};

use crate::database::{Database, Domain, DomainInteraction};
pub struct DomainRenderer<'d> {
    list: &'d Vec<Domain>,
    selected: Option<&'d Domain>,
}

impl<'d> DomainRenderer<'d> {
    pub fn new(list: &'d Vec<Domain>, selected: Option<&'d Domain>) -> Self {
        Self { list, selected }
    }

    pub fn render(
        &self,
        db: &Database,
        screen: &mut Frame<CrosstermBackend<Stdout>>,
        container: Rect,
        list_state: &mut ListState,
    ) {
        if self.list.is_empty() {
            let domain_ui_panes = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(container);

            screen.render_widget(self.render_empty_list_widget(), domain_ui_panes[0]);
            return;
        }

        let selected = self.selected.unwrap();

        if selected.id == 0 {
            let domain_ui_panes = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(container);

            let domain_details_panes = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(domain_ui_panes[1]);

            screen.render_stateful_widget(
                self.render_list_widget(),
                domain_ui_panes[0],
                list_state,
            );
            screen.render_widget(self.render_empty_info_widget(), domain_details_panes[0]);

            return;
        }

        let selected_domain = self.selected.unwrap();
        let domain_ui_panes = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(container);

        let domain_details_panes = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
            .split(domain_ui_panes[1]);

        let selected_domain_info = db
            .get_info(&selected_domain)
            .expect("get information from domain");

        let domain_interaction = db.domain_interaction(&selected_domain);

        screen.render_stateful_widget(self.render_list_widget(), domain_ui_panes[0], list_state);
        screen.render_widget(
            self.render_info_widget(selected_domain_info),
            domain_details_panes[0],
        );
        screen.render_widget(
            self.render_interaction_widget(domain_interaction),
            domain_details_panes[1],
        );
    }

    pub fn render_list_widget(&self) -> List<'_> {
        let list_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White))
            .title("Domains")
            .border_type(BorderType::Plain);

        let list_items: Vec<_> = self
            .list
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

    fn render_info_widget(&self, domain: Domain) -> Table<'_> {
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

    fn render_empty_list_widget(&self) -> Paragraph<'d> {
        Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "The domain database is currently empty.",
                Style::default().fg(Color::Red),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw(
                "Navigate to a domain of interest in Safari to initialize it.",
            )]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("No domains available")
                .border_type(BorderType::Plain),
        )
    }

    fn render_empty_info_widget(&self) -> Paragraph<'d> {
        Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "No information available for the selected domain.",
                Style::default().fg(Color::Red),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw(
                "Navigate to the domain of interest in Safari to initialize it.",
            )]),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("No domains available")
                .border_type(BorderType::Plain),
        )
    }

    fn render_interaction_widget(&self, interaction: DomainInteraction) -> Table<'d> {
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
}
