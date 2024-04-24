use std::fs;

const HEADER_ROWS: usize = 3;
const NAME_INDEX: usize = 2;
const WING_INDEX: usize = 3;
const LINK_INDEX: usize = 4;
const MIDDLE_INDEX: usize = 5;

#[derive(Debug)]
struct Player {
    name: String,
    wing: u32,
    link: u32,
    middle: u32,
}

fn main() {
    let val: String = match fs::read_to_string("./positions.csv") {
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
                    _ => 0,
                },
                link: match row[LINK_INDEX].parse::<u32>() {
                    Ok(n) => n,
                    _ => 0,
                },
                middle: match row[MIDDLE_INDEX].parse::<u32>() {
                    Ok(n) => n,
                    _ => 0,
                },
            });
        }
    }
    dbg!(cells);
}
