use std::fs;

const HEADER_ROWS: usize = 1;
const NAME_INDEX: usize = 1;
const WING_INDEX: usize = 2;
const LINK_INDEX: usize = 3;
const MIDDLE_INDEX: usize = 4;
const POSITIONS_CSV: &str = "./positions.csv";

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
    let wings: Vec<Player> = sort_by_position(cells.clone(), Position::Wing);
    let links: Vec<Player> = sort_by_position(cells.clone(), Position::Link);
    let middles: Vec<Player> = sort_by_position(cells.clone(), Position::Middle);

    println!("WINGS");
    for player in wings {
        println!("\t{} - {}", player.name, player.wing)
    }
    println!("LINKS");
    for player in links {
        println!("\t{} - {}", player.name, player.link)
    }
    println!("MIDDLES");
    for player in middles {
        println!("\t{} - {}", player.name, player.middle)
    }
}

fn sort_by_position(input: Vec<Player>, position: Position) -> Vec<Player> {
    let get_value: &dyn Fn(&Player) -> u32 = &|p: &Player| match position {
        Position::Wing => p.wing,
        Position::Link => p.link,
        Position::Middle => p.middle,
    };
    let mut intermediary: Vec<Player> = input
        .into_iter()
        .filter(|p: &Player| get_value(p) < u32::MAX)
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
            .filter(|c: &char| c != &'"')
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
