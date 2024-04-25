use std::{fmt, fs};

const HEADER_ROWS: usize = 1;
const NAME_INDEX: usize = 1;
const WING_INDEX: usize = 2;
const LINK_INDEX: usize = 3;
const MIDDLE_INDEX: usize = 4;
const POSITIONS_CSV: &str = "./positions.csv";
const FIRST_CHOICES_ONLY: bool = true;

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
        }
    }
}

fn main() {
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
    let wings: Vec<Player> = sort_by_position(cells.clone(), Position::Wing, FIRST_CHOICES_ONLY);
    let links: Vec<Player> = sort_by_position(cells.clone(), Position::Link, FIRST_CHOICES_ONLY);
    let middles: Vec<Player> = sort_by_position(cells.clone(), Position::Middle, FIRST_CHOICES_ONLY);

    print_players_in_position(wings, Position::Wing);
    print_players_in_position(links, Position::Link);
    print_players_in_position(middles, Position::Middle);
}

fn print_players_in_position(players: Vec<Player>, position: Position) -> () {
    println!("{}", position);
    for player in players {
        let position_preference = match position {
            Position::Wing => player.wing,
            Position::Link => player.link,
            Position::Middle => player.middle,
        };
        println!("\t{} - {}", player.name, position_preference)
    }
}

fn sort_by_position(input: Vec<Player>, position: Position, only_first_choices: bool) -> Vec<Player> {
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
        name: row[NAME_INDEX]
            .chars()
            .collect::<String>(),
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
