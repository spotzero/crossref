use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: crossref <source file> <file to search>");
        exit(1);
    }

    let source = file_to_vec(&args[1]);
    let search = file_to_vec(&args[2]);
    for i in 0..source.len() {
        println!("{}:", source[i]);
        let mut matched = false;
        for j in 0..search.len() {
            let matches: Vec<&str> = search[j].matches(source[i].as_str()).collect();
            if matches.len() > 0 {
                println!("\t{}: {}", j, search[j]);
                matched = true;
            }
        }
        if !matched {
            println!("\tNo matches");
        }
    }
}

fn file_to_vec(filename: &String) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();

    let source_file = File::open(filename).expect(format!("Couldn't open {}", filename.as_str()).as_str());
    for line in BufReader::new(&source_file).lines() {
        v.push(String::from(line.expect("Error reading file").to_lowercase().trim()));
    }
    v
}
