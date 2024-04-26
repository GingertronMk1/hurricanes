use eframe::{
    egui::{self, Color32, Pos2, Shape, Ui, Vec2},
    CreationContext, NativeOptions,
};
use std::{cmp::Ordering, fmt, fs};

const HEADER_ROWS: usize = 1;
const NAME_INDEX: usize = 1;
const WING_INDEX: usize = 2;
const LINK_INDEX: usize = 3;
const MIDDLE_INDEX: usize = 4;
const POSITIONS_CSV: &str = "./positions.csv";
const FIRST_CHOICES_ONLY: bool = false;

#[derive(Debug, Clone)]
struct Player {
    name: String,
    wing: u32,
    link: u32,
    middle: u32,
    present: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: String::from("Default"),
            wing: u32::MAX,
            link: u32::MAX,
            middle: u32::MAX,
            present: true,
        }
    }
}

impl Player {
    fn get_position_value(&self, position: Position) -> u32 {
        match position {
            Position::Wing => self.wing,
            Position::Link => self.link,
            Position::Middle => self.middle
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Position {
    Wing,
    Link,
    Middle,
}

impl Position {
    fn list() -> Vec<Self> {
        return Vec::from([Self::Wing, Self::Link, Self::Middle]);
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{}",
            match self {
                Position::Wing => "Wing",
                Position::Link => "Link",
                Position::Middle => "Middle",
            }
        );
    }
}

struct MyEguiApp {
    players: Vec<Player>,
    left_wing: Shape,
    left_link: Shape,
    left_middle: Shape,
    right_middle: Shape,
    right_link: Shape,
    right_wing: Shape,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        let new_circle = |x: i16, y: i16| {
            Shape::circle_filled(
                Pos2::new(f32::from(x), f32::from(y)),
                50.0,
                Color32::from_rgb(255, 0, 0),
            )
        };
        return Self {
            players: Vec::new(),
            left_wing: new_circle(100, 500),
            left_link: new_circle(200, 500),
            left_middle: new_circle(300, 500),
            right_middle: new_circle(400, 500),
            right_link: new_circle(500, 500),
            right_wing: new_circle(600, 500),
        };
    }
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
        return MyEguiApp {
            players: cells,
            ..Default::default()
        };
    }

    fn get_field_players(&self) -> Vec<Shape> {
        return Vec::from([
            self.left_wing.clone(),
            self.left_link.clone(),
            self.left_middle.clone(),
            self.right_middle.clone(),
            self.right_link.clone(),
            self.right_wing.clone(),
        ]);
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut Ui| {
            // for player in self.get_field_players() {
            //     ui.painter().add(player);
            // }
            ui.horizontal(|ui: &mut Ui| {
                ui.group(|ui: &mut Ui| {
                    ui.vertical(|ui: &mut Ui| {
                        ui.heading("All Players");
                        for player in &mut self.players {
                            ui.checkbox(&mut player.present, player.name.clone());
                        }
                    });
                });
                ui.vertical(|ui| {
                    for position in Position::list() {
                        ui.group(|ui: &mut Ui| {
                            ui.vertical(|ui: &mut Ui| {
                                ui.heading(position.to_string());
                                let pos_players: Vec<Player> =
                                    sort_by_position(self.players.clone(), position);
                                for (n, player) in pos_players.iter().enumerate() {
                                    if n > 0 && pos_players[n - 1].get_position_value(position) != player.get_position_value(position) {
                                        ui.separator();
                                    }
                                    ui.label(&player.name);
                                }
                            });
                        });
                    }
                });
            });
        });
    }
}

fn sort_by_position(input: Vec<Player>, position: Position) -> Vec<Player> {
    let max_preference = if FIRST_CHOICES_ONLY { 1 } else { u32::MAX - 1 };
    let mut intermediary: Vec<Player> = input
        .into_iter()
        .filter(|p: &Player| p.get_position_value(position) <= max_preference && p.present)
        .collect();
    intermediary.sort_by(|p1: &Player, p2: &Player| {
        if p1.get_position_value(position).cmp(&p2.get_position_value(position)) == Ordering::Equal {
            p1.name.cmp(&p2.name)
        } else {
           p1.get_position_value(position).cmp(&p2.get_position_value(position))
        }
    });
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
        present: true,
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

fn main() {
    let native_options: NativeOptions = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            resizable: Some(false),
            inner_size: Some(Vec2::from([1000.0, 1000.0])),
            ..egui::ViewportBuilder::default()
        },
        ..Default::default()
    };
    let _ = eframe::run_native(
        "Touch Rugby Helper",
        native_options,
        Box::new(|cc: &CreationContext| Box::new(MyEguiApp::new(cc))),
    );
}
