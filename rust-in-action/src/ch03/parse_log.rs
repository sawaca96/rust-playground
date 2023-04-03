#![allow(dead_code)]
#[derive(Debug)]
enum Event {
    Update,
    Delete,
    Unkonwn,
}

type Message = String;

fn parse_log(line: &'static str) -> (Event, Message) {
    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    if parts.len() == 1 {
        return (Event::Unkonwn, String::from(line));
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" => (Event::Update, rest),
        "DELETE" => (Event::Delete, rest),
        _ => (Event::Unkonwn, String::from(line)),
    }
}
pub fn run() {
    let log = "BEGIN Transcation XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:L0/22111";

    for line in log.lines() {
        let (event, message) = parse_log(line);
        println!("{:?} {}", event, message);
    }
}
