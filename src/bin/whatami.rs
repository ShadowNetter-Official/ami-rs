use std::env;

fn main() {
    let messages = ["A programmer", "A computer", "A student", "Idk gng 🥀", "An  user"];
    let default_message: String = "A human".to_string();
    let mut message: String = String::new();
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        message = default_message;
    }
    else {
        match &argv[1] as &str {
            "-r" | "--random" => message = messages[fastrand::usize(..messages.len())].to_string(), // random use case
            "-h" | "--help" => help(), // help message
            &_ => help(),
        }
    }
    println!("{}", message);
}

fn help() {
    println!("whatami | Tired of whoami? Try whatami instead\n");
    println!("usage: \n");
    println!("whatami              | outputs what you are (default: A human)");
    println!("whatami -r, --random | outputs what you are (randomized)");
    println!("whatami -h, --help   | prints this message");
}
