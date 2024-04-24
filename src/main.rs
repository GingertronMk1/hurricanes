use std::{cmp::Ordering, fs};

const HEADER_ROWS: usize = 3;
const NAME_INDEX: usize = 2;
const WING_INDEX: usize = 3;
const LINK_INDEX: usize = 4;
const MIDDLE_INDEX: usize = 5;
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
        let row: Vec<&str> = line.split(",").collect::<Vec<&str>>();
        if row.len() > NAME_INDEX && row[NAME_INDEX].len() > 0 {
            cells.push(Player {
                name: row[NAME_INDEX].to_string(),
                wing: match row[WING_INDEX].parse::<u32>() {
                    Ok(n) => n,
                    _ => u32::MAX,
                },
                link: match row[LINK_INDEX].parse::<u32>() {
                    Ok(n) => n,
                    _ => u32::MAX,
                },
                middle: match row[MIDDLE_INDEX].parse::<u32>() {
                    Ok(n) => n,
                    _ => u32::MAX,
                },
            });
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
    intermediary.sort_by(|p1: &Player, p2: &Player| {
        if get_value(p1) > get_value(p2) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });
    return intermediary.clone();
}
