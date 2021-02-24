use std::{thread, time};
use std::sync::mpsc;
use std::path::Path;
use std::sync::mpsc::{Sender, Receiver};
use anyhow::{ Result};

mod actions;
mod commands;


fn main() -> Result<()>{
    
    // Check that this is a git-repo.
    let is_git_repo = Path::new(".git").exists();
    if !is_git_repo {
        println!("There is no `.git`-folder. Run this program in the root of the repo");
        return Ok(())
    }
    // TODO: Commands 
    let repo = commands::pwd_command();
    
    // Clear screen and move to top
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    let (tx, rx): (Sender<gf::Event>, Receiver<gf::Event>) = mpsc::channel();
    
    // Create thread to periodically update git-status
    let status_tx = tx.clone();
    thread::spawn(move || {
        let reload_time = time::Duration::from_millis(3 * 1000); // three seconds
        let mut prev_status: String = "".to_string(); 
        loop {
            let status = commands::git_status();
            if status != prev_status {
                prev_status = status.clone();
                let event = gf::Event { msg: gf::Message::STATUS, data: status};
                status_tx.send(event).unwrap();
            }
            thread::sleep(reload_time);
        }
    });

    // Create thread to listen to input from user.
    let input_tx = tx.clone();
    thread::spawn(move || {
        loop {
            thread::sleep(time::Duration::from_millis(10));
            let event = actions::print_actions();
            input_tx.send(event).unwrap();
        }
    });

    // Loop to receive events from channel to rerender
    let mut status = String::new();
    loop {
        let new_event = rx.recv().unwrap();
        // println!("{:?}", new_event);
        match new_event {
            gf::Event { msg: gf::Message::STATUS, data } => {
                status = data.clone();
                gf::print_status(&*repo, status.clone()); // <-- TODO: Cleaner ?
            },
            gf::Event { msg: gf::Message::DIFF, data } => {
                gf::print_status(&*repo, status.clone());
                println!("Output: ");
                gf::print_diff(data.clone());
            },
            gf::Event { msg: gf::Message::COMMAND, data } => {
                gf::print_status(&*repo, status.clone());
                println!("Output: ");
                println!("{}", data);
            },
            gf::Event { msg: gf::Message::INPUT, data: _ } => {
                gf::print_status(&*repo, status.clone());
            },
            gf::Event { msg: gf::Message::OTHER, data: _ } => {
                // Do nothing 
            },
        }
    }
}

