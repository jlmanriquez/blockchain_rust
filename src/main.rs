use crate::blockchain::Blockchain;
use clap::{App, Arg};
use std::io::{stdin, stdout, Error, Write};

mod block;
mod blockchain;

fn main() {
    let matches = App::new("Blockchain Example")
        .version("1.0")
        .author("jlmanriquez")
        .about("A blockchain example taking as example an existing implementation in C++")
        .arg(Arg::with_name("difficulty")
            .short("d")
            .long("difficulty")
            .value_name("difficulty")
            .help("Difficulty use for mine block. Default value is 3")
            .required(false)
            .takes_value(true))
        .get_matches();

    let difficulty = matches.value_of("difficulty")
        .unwrap_or("3")
        .parse::<u32>()
        .unwrap();

    let mut b_chain = Blockchain::new(difficulty);

    show_menu(&mut b_chain).unwrap();
}

fn show_menu(b_chain: &mut Blockchain) -> Result<(), Error> {
    println!("---------------------");
    println!("Options:");
    println!("1. Add block");
    println!("2. Show Blockchain");
    println!("3. Exit");
    print!("> ");

    stdout().flush().unwrap();

    let option = &mut String::new();
    stdin().read_line(option)?;

    match option.trim() {
        "1" => {
            add_block(b_chain);
            show_menu(b_chain).unwrap();
        },
        "2" => {
            show_blockchain(b_chain);
            show_menu(b_chain).unwrap();
        },
        "3" => (),
        _ => { show_menu(b_chain).unwrap(); },
    }

    Ok(())
}

fn add_block(b_chain: &mut Blockchain) {
    print!("\nEnter data: ");
    stdout().flush().unwrap();

    let data = &mut String::new();
    stdin().read_line(data).unwrap();

    b_chain.add_block(data.as_str());
}

fn show_blockchain(b_chain: &Blockchain) {
    b_chain.iter(|b| {
        println!("{}", b.get_hash());
    })
}
