use termion::{self, color};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    COMMAND,
    DIFF,
    INPUT,
    OTHER,
    STATUS,
}

#[derive(Debug )]
pub struct Event {
    pub msg: Message,
    pub data: String,
}

pub fn print_diff(diff: String) {
    let lines: Vec<&str> = diff.split('\n').collect();
    for  line in lines {
        if line.len() < 1 {
            continue
        }
        let first_char = line.chars().nth(0).unwrap();
        match first_char {
            '+' => print!("{}", color::Fg(color::Green)),
            '-' => print!("{}", color::Fg(color::Red)),
            '@' => print!("{}", color::Fg(color::Magenta)),
            'd' => print!("{}", color::Fg(color::Yellow)),
            'i' => print!("{}", color::Fg(color::Yellow)),
            _ => print!("{}", color::Fg(color::Reset)),
        };
        println!("{}{}", line, color::Fg(color::Reset));
    }
    println!("");
}

pub fn print_status(repo: &str, status: String) {
    // Clear screen and move to top
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
    print!("{}[3J", 27 as char); // Also delete scrollback history.
    println!("Status ({}):\t(A=add, M=modified, R=rename, D=deleted, ??=not-tracked)", repo);

    let lines: Vec<&str> = status.split('\n').collect();
    if lines.len() <= 1 {
        println!("No changes in the repo ☀️");
        return;
    }
    for (index, line) in lines.iter().enumerate() {
        if line.len() < 2 {
            continue
        }
        print!("{}: ", index);
        let change = &line[..2];
        // Apparently, it is to the left if it is staged, to the right if
        // the change is unstaged.
        match change {
            "A " => print!("{}", color::Fg(color::Green)),
            "M " => print!("{}", color::Fg(color::Green)),
            "AM" => print!("{}", color::Fg(color::Yellow)),
            " M" => print!("{}", color::Fg(color::Yellow)),
            "R " => print!("{}", color::Fg(color::Yellow)),
            " D" => print!("{}", color::Fg(color::Red)),
            "??" => print!("{}", color::Fg(color::Red)),
            _ => print!("{}", color::Fg(color::Reset)),
        };
        println!("{}{}", line, color::Fg(color::Reset));
    }
    println!("");
}

