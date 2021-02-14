use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use serde::Deserialize;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use std::{cmp, path::PathBuf};
use structopt::StructOpt;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Terminal,
};

mod database;
use database::Database;

mod gui;
use gui::Gui;

const DATABASE_PATH: &'static str = "Library/Containers/com.apple.Safari/Data/Library/WebKit/WebsiteData/ResourceLoadStatistics/observations.db";

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
/// ITP TL;DR; the tool you didn't know you needed to understand ITP.
struct Opts {
    /// Safari's SQLite path
    #[structopt(short, long)]
    path: Option<PathBuf>,
    /// A list of comma separated domains.
    #[structopt(short, long, use_delimiter = true)]
    pub domains: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub path: Option<PathBuf>,
    domains: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = fetch_config();
    let db = Database::connect(config).expect("Couldn't connect to the database");

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
            let main_panes = Gui::render_panes(screen.size());
            let tabs_widget = Gui::render_tabs(&menu_titles, active_menu_item.into());
            let copyright_widget = Gui::render_copyright();

            screen.render_widget(tabs_widget, main_panes[0]);

            match active_menu_item {
                MenuItem::Home => screen.render_widget(Gui::render_home_pane(), main_panes[1]),
                MenuItem::Domains => {
                    let domain_list = db.get_domains().expect("fetch domain list");
                    let selected_domain = domain_list
                        .get(domain_list_state.selected().expect("domain to be selected"))
                        .expect("domain exists");

                    let selected_domain_info = db
                        .get_info(&selected_domain)
                        .expect("get information from domain");

                    let domain_interaction = db.domain_interaction(&selected_domain);

                    let domain_ui_panes = Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                        )
                        .split(main_panes[1]);

                    let domain_details_panes = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(
                            [Constraint::Percentage(15), Constraint::Percentage(85)].as_ref(),
                        )
                        .split(domain_ui_panes[1]);

                    let domain_list_widget = Gui::render_domain_list_widget(&domain_list);
                    let domain_details_table_widget =
                        Gui::render_domain_info_widget(&selected_domain_info);
                    let domain_interaction_widget =
                        Gui::render_domain_interaction_widget(domain_interaction);

                    screen.render_stateful_widget(
                        domain_list_widget,
                        domain_ui_panes[0],
                        &mut domain_list_state,
                    );
                    screen.render_widget(domain_details_table_widget, domain_details_panes[0]);
                    screen.render_widget(domain_interaction_widget, domain_details_panes[1]);
                }
            }

            screen.render_widget(copyright_widget, main_panes[2]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Char('h') | KeyCode::Char('H') => active_menu_item = MenuItem::Home,
                KeyCode::Char('d') | KeyCode::Char('D') => active_menu_item = MenuItem::Domains,
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
                KeyCode::Char('q') | KeyCode::Char('Q') => {
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

fn fetch_config() -> Config {
    let opts = Opts::from_args();

    let mut config = match read_config() {
        Some(config) => {
            let mut config: Config = toml::from_str(&config).expect("config to follow TOML format");

            if opts.path.is_some() {
                config.path = opts.path;
            }

            if opts.domains.is_some() {
                config.domains = opts.domains;
            }

            config
        }
        None => Config {
            path: opts.path,
            domains: opts.domains,
        },
    };

    if config.path.is_none() {
        let mut db_path = match dirs::home_dir() {
            Some(dir) => dir,
            None => panic!("Could not infer home directory."),
        };
        db_path.push(DATABASE_PATH);
        config.path = Some(db_path);
    }

    config
}

fn read_config() -> Option<String> {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".itprc");

    if config_path.exists() {
        Some(std::fs::read_to_string(config_path).expect("able to read from ~/.itprc"))
    } else {
        None
    }
}
