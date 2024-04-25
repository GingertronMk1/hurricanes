use eframe::{egui, CreationContext};
use std::{fmt, fs};

const HEADER_ROWS: usize = 1;
const NAME_INDEX: usize = 1;
const WING_INDEX: usize = 2;
const LINK_INDEX: usize = 3;
const MIDDLE_INDEX: usize = 4;
const POSITIONS_CSV: &str = "./positions.csv";
const FIRST_CHOICES_ONLY: bool = true;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Touch Rugby Helper",
        native_options,
        Box::new(|cc: &CreationContext| Box::new(MyEguiApp::new(cc))),
    );
}

#[derive(Default)]
struct MyEguiApp {
    players: Vec<Player>,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let val: String = match fs::read_to_string(POSITIONS_CSV) {
            Ok(s) => s,
            Err(s) => panic!("{}", s),
        };
        let mut cells: Vec<Player> = Vec::new();
        let lines_in_val: Vec<&str> = val.split(|c| c == '\n' || c == '\r').collect();
        for line in &lines_in_val[HEADER_ROWS..] {
            let row: Vec<&str> = line.split(",").map(|s| s.trim_matches('"')).collect();
            if row.len() > NAME_INDEX && row[NAME_INDEX].len() > 0 {
                cells.push(player_from_row(row));
            }
        }
        return Self { players: cells };
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                for position in [Position::Wing, Position::Link, Position::Middle] {
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            ui.heading(position.to_string());
                            for player in
                                sort_by_position(self.players.clone(), position, FIRST_CHOICES_ONLY)
                            {
                                ui.label(player.name);
                            }
                        });
                    });
                }
            });
        });
    }
}

#[derive(Debug, Clone)]
struct Player {
    name: String,
    wing: u32,
    link: u32,
    middle: u32,
}

#[derive(Debug, Clone)]
enum Position {
    Wing,
    Link,
    Middle,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            Position::Wing => write!(f, "Wing"),
            Position::Link => write!(f, "Link"),
            Position::Middle => write!(f, "Middle"),
        };
    }
}

fn sort_by_position(
    input: Vec<Player>,
    position: Position,
    only_first_choices: bool,
) -> Vec<Player> {
    let get_value: &dyn Fn(&Player) -> u32 = &|p: &Player| match position {
        Position::Wing => p.wing,
        Position::Link => p.link,
        Position::Middle => p.middle,
    };

    let max_preference = if only_first_choices { 1 } else { u32::MAX - 1 };
    let mut intermediary: Vec<Player> = input
        .into_iter()
        .filter(|p: &Player| get_value(p) <= max_preference)
        .collect();
    intermediary.sort_by(|p1: &Player, p2: &Player| get_value(p1).cmp(&get_value(p2)));
    return intermediary;
}

fn player_from_row(row: Vec<&str>) -> Player {
    let mut wing: u32 = parse_position_value(row[WING_INDEX]);
    let mut link: u32 = parse_position_value(row[LINK_INDEX]);
    let mut middle: u32 = parse_position_value(row[MIDDLE_INDEX]);
    if wing == link && link == middle {
        wing = 1;
        link = 1;
        middle = 1;
    }
    return Player {
        name: row[NAME_INDEX].chars().collect::<String>(),
        wing,
        link,
        middle,
    };
}

fn parse_position_value(input: &str) -> u32 {
    return match input
        .chars()
        .filter(|c: &char| (*c).is_alphanumeric())
        .collect::<String>()
        .parse::<u32>()
    {
        Ok(n) => n,
        Err(_) => u32::MAX,
    };
}
