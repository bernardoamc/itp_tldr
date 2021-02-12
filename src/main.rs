use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Terminal,
};

use std::cmp;

mod database;
use database::Database;

mod gui;
use gui::Gui;

use structopt::StructOpt;

enum Event<I> {
    Input(I),
    Tick,
}
#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Domains,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Domains => 1,
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "itp_tldr", setting = structopt::clap::AppSettings::TrailingVarArg)]
/// ITP TL;DR; the tool you didn't know you need to understand ITP.
pub struct Opts {
    /// A list of comma separated domains.
    #[structopt(short, long, use_delimiter = true)]
    pub domains: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::from_args();
    let db = Database::connect(opts.domains).expect("Couldn't connect to the database");

    enable_raw_mode().expect("can run in raw mode");

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_secs(1);

    thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("Timeout polling to work") {
                if let CEvent::Key(key) = event::read().expect("Event reading is possible") {
                    tx.send(Event::Input(key))
                        .expect("Sending events is possible");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let menu_titles = vec!["Home", "Domains"];
    let mut active_menu_item = MenuItem::Home;
    let mut domain_list_state = ListState::default();
    domain_list_state.select(Some(0));

    loop {
        terminal.draw(|screen| {
            let panes = Gui::render_panes(screen.size());
            let tabs = Gui::render_tabs(&menu_titles, active_menu_item.into());
            let copyright = Gui::render_copyright();

            screen.render_widget(tabs, panes[0]);

            match active_menu_item {
                MenuItem::Home => screen.render_widget(Gui::render_home_pane(), panes[1]),
                MenuItem::Domains => {
                    let domain_ui = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(panes[1]);

                    let (domain_list, domain_details) =
                        Gui::render_domains(&db, &domain_list_state);

                    screen.render_stateful_widget(
                        domain_list,
                        domain_ui[0],
                        &mut domain_list_state,
                    );
                    screen.render_widget(domain_details, domain_ui[1]);
                }
            }

            screen.render_widget(copyright, panes[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('h') => active_menu_item = MenuItem::Home,
                KeyCode::Char('d') => active_menu_item = MenuItem::Domains,
                KeyCode::Down => {
                    if let Some(selected) = domain_list_state.selected() {
                        let amount_domains = db.domains_len().expect("failed to count domains");
                        let index = cmp::min(selected as i32 + 1, amount_domains - 1);
                        domain_list_state.select(Some(index as usize));
                    }
                }
                KeyCode::Up => {
                    if let Some(selected) = domain_list_state.selected() {
                        let index = cmp::max(selected as i32 - 1, 0);
                        domain_list_state.select(Some(index as usize));
                    }
                }
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    terminal.clear()?;
                    break;
                }
                _ => {}
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
