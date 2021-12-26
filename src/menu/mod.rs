use std::{sync::mpsc::{Sender, Receiver}};

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture, Event, self, KeyCode}};
use tui::{widgets::{Block, Borders, List, ListItem}, backend::{CrosstermBackend, Backend}, Terminal, Frame, layout::{Layout, Constraint, Direction}, style::{Style, Color, Modifier}};

use crate::core::structs::{ThreadMsg, Config, PlayerData};

use nom::{named, map_res, take_until_and_consume};

use self::stateful_list::StatefulList;

mod stateful_list;

named!(
    pub parse_string<&str>,
    map_res!(take_until_and_consume!("\0"), ::std::str::from_utf8)
);

pub struct Menu<'a> {
    settings: StatefulList<&'a str>,
    config: Config,
    player_data: [Option<PlayerData>; 64],
    average_execution_time: u128
}

impl<'a> Menu<'a> {
    fn new() -> Menu<'a> {
        Menu {
            settings: StatefulList::with_items(vec!["glow",]),
            config: Config::default(),
            player_data: [None; 64],
            average_execution_time: 0,
        }
    }
    pub fn run<B: Backend>(&mut self, f: &mut Frame<B>) {
        let rects = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
            ])
            .margin(1)
            .split(f.size());

        let settings = get_settings_block(&self.config, &self.settings);
        let player_info = get_player_block(self.player_data.clone());
        let statistics = get_statistics_block(self.average_execution_time);

        f.render_stateful_widget(settings, rects[0], &mut self.settings.state);
        f.render_widget(player_info, rects[1]);
        f.render_widget(statistics, rects[2]);
    }
}

pub fn run_menu(menu_rx: Receiver<ThreadMsg>, menu_tx: Sender<ThreadMsg>) -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut menu = Menu::new();
    menu.settings.next();
    terminal.clear().unwrap();
    
    loop {
        terminal.draw(|f| {
            menu.run(f);
        }).unwrap();

        if crossterm::event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => menu.settings.next(),
                    KeyCode::Up => menu.settings.previous(),
                    KeyCode::Left => menu.settings.change_config(-1, &mut menu.config),
                    KeyCode::Right => menu.settings.change_config(1, &mut menu.config),
                    _ => {}
                }
            }
        }

        let response = menu_rx.try_recv();
        if response.is_ok() {
            let msg = response.unwrap();

            if msg.average_execution_time.is_some() {
                menu.average_execution_time = msg.average_execution_time.unwrap()
            }

            menu.player_data = msg.playerdata_array;
        }

        menu_tx.send(ThreadMsg { 
            exited: false,
            new_config: Some(menu.config),
            playerdata_array: [None; 64],
            average_execution_time: None
        }).unwrap();
    }

    menu_tx.send(ThreadMsg { 
        exited: true,
        new_config: None,
        playerdata_array: [None; 64],
        average_execution_time: None
    }).unwrap();

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

fn get_player_block(player_data: [Option<PlayerData>; 64]) -> List<'static> {
    let items = player_data.clone();

    let player_data = items
        .iter()
        .filter(|data| data.is_some());

    let items: Vec<ListItem> = player_data.map(|data| {
        let name = parse_string(&data.unwrap().name).ok().unwrap()
            .1
            .to_string();

        ListItem::new(format!("{}", name))
            .style(
                Style::default()
                    .fg(Color::White)
            )
    }).collect();

    List::new(items)
        .block(
            Block::default()
                .title("player info")
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(Color::LightCyan)
                )
        )
}

fn get_settings_block<T: std::cmp::PartialEq + std::fmt::Display>(config: &Config, list_state: &StatefulList<T>) -> List<'static> {
    let items: Vec<ListItem> = list_state
        .items
        .iter()
        .map(|i| {
            let index = list_state.items.iter().position(|item| item == i).unwrap();
            let name = config.valuestr_at_index(index).unwrap();

            ListItem::new(format!("{} - {}", i, name))
                .style(
                    Style::default()
                        .fg(Color::White)
                )
        })
        .collect();

    List::new(items)
        .block(
            Block::default()
                .title("settings")
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(Color::LightCyan)
                )
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}

fn get_statistics_block(average_execution_time: u128) -> List<'static> {
    let items = vec![
        ListItem::new(format!("execution time: {} ns", average_execution_time))
            .style(
                Style::default()
                .fg(Color::White)
            ),

        ListItem::new(" "),

        ListItem::new("Press q to quit.")
            .style(
                Style::default()
                .fg(Color::White)
            )
    ];

    List::new(items)
        .block(
            Block::default()
                .title("info")
                .borders(Borders::ALL)
                .border_style(
                    Style::default()
                        .fg(Color::LightCyan)
                )
        )
        
}
