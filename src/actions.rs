use regex::Regex;
use read_input::prelude::*;


use crate::commands;

pub fn print_actions () -> gf::Event {
    println!("(? for help)");
    let action : String = input().msg("Action: ").get();

    // Check if command is help (`?`)
    let question_re = Regex::new(r"\s*\?\s*").unwrap();
    if  question_re.is_match(&action) { 
        println!("COMMANDS: ");
        println!(" d $INDEX --> By writing `d 1` you do a git diff on the ");
        println!("              file with index 1. The indexes are shown  ");
        println!("              in the file-list above. The space can be  ");
        println!("              ommitted to `d1`  ");
        println!("                ");
        println!(" git ...  --> All lines starting with `g` or `git` will ");
        println!("              be treated as git commands and will run   ");
        println!("              git in the background and show it's output.");
        println!("              E.g: `glog --1`, `git fetch -v`, `g checkout master`");
        let event = gf::Event { msg: gf::Message::OTHER, data: String::new() };
        return event;
    }

    // Check if command is a diff-command
    let diff_re = Regex::new(r"d{0,1}\s*\d+").unwrap();
    let is_diff = diff_re.is_match(&action);
    if is_diff {
        // REGEX: Ugly code to run a git diff
        let re = Regex::new(r"d{0,1}\s*(\d+)").unwrap();
        let capture = re.captures(&action).unwrap();
        let index: usize  = capture[1].parse::<usize>().unwrap();

        // Get list of files and match with index
        let status = commands::git_status();
        let lines: Vec<&str> = status.split('\n').collect();
        let mut line = lines[index].to_string();
        let file_name = line.split_off(3);

        // Run diff command
        let diff_data = commands::git_diff(file_name);

        // Return output 
        let event = gf::Event { msg: gf::Message::DIFF, data: diff_data};
        return event;
    } 

    // Check if is a git-command
    let git_re = Regex::new(r"g(it){0,1}\s{0,1}").unwrap();
    let is_git_cmd = git_re.is_match(&action);
    if  is_git_cmd {
        // Regex to parse command
        let re = Regex::new(r"g(it){0,1}\s{0,1}(.*)").unwrap();
        let capture = re.captures(&action).unwrap();
        let c = &capture[2];

        // Parse to arguments and run command
        let args: Vec<String> = c.split(' ').map(String::from).collect();
        println!("ARGS: {:?}", args);
        let output = commands::git_command(args);

        // Return output
        let event = gf::Event { msg: gf::Message::COMMAND, data: output};
        return event;
    }

    let event = gf::Event { msg: gf::Message::INPUT, data: action};
    return event;
}
