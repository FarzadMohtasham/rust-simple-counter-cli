use std::process::exit;
use std::{thread, time};
use clap::{Arg, command};
use clap::builder::TypedValueParser;

fn main() {
    let counter_command_match_result = command!()
        .about("This a simple rusty counter cli app")
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .required(true)
                .help("Take a number and starts counting from it every one second")
        )
        .arg(
            Arg::new("sleep")
                .short('s')
                .long("sleep")
                .help("Sleeps after counting | Default is 1sec")
        )
        .get_matches();

    let mut counter = match counter_command_match_result.get_one::<String>("count").unwrap().parse::<i32>() {
        Ok(v) => v,
        Err(_) => {
            println!("Error parsing argument, Please enter a valid number!");
            exit(1);
        }
    };

    let sleep: u32 = match counter_command_match_result.get_one::<String>("sleep").unwrap().parse::<u32>() {
        Ok(v) => v,
        Err(_) => {
            println!("Error parsing argument, Please enter a valid number!");
            exit(1);
        }
    };

    println!("Counter starts from {:?}", counter);

    while true {
        println!("counter => {}", counter);
        counter += 1;
        thread::sleep(time::Duration::from_secs(sleep as u64));
    }
}