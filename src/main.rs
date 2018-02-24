extern crate clap;

use std::fs::File;
use std::io::{BufReader,BufRead};
use clap::{Arg, App};

/// Options for print
#[derive(Debug)]
struct PrintOptions {
    number_nonblank: bool,
    show_ends: bool,
    number_line: bool,
    squeeze_blank: bool,
    show_tabs: bool,
    show_nonprinting: bool
}

fn main() {
    let matches = App::new("ccat")
                        .version("0.1.0")
                        .author("Guangming Mao <maogm12@gmail.com>")
                        .about("Colorized cat")
                        .arg(Arg::with_name("show-all")
                            .short("A")
                            .long("show-all")
                            .help("equivalent to -vET"))
                        .arg(Arg::with_name("number-nonblank")
                            .short("b")
                            .long("number-nonblank")
                            .help("number nonempty output lines"))
                        .arg(Arg::with_name("e")
                            .short("e")
                            .help("equivalent to -vE"))
                        .arg(Arg::with_name("show-ends")
                            .short("E")
                            .long("show-ends")
                            .help("display $ at end of each line"))
                        .arg(Arg::with_name("number")
                            .short("n")
                            .long("number")
                            .help("number all output lines"))
                        .arg(Arg::with_name("squeeze-blank")
                            .short("s")
                            .long("squeeze-blank")
                            .help("suppress repeated empty output lines"))
                        .arg(Arg::with_name("t")
                            .short("t")
                            .help("equivalent to -vT"))
                        .arg(Arg::with_name("show-tabs")
                            .short("T")
                            .long("show-tabs")
                            .help("display TAB characters as ^I"))
                        .arg(Arg::with_name("show-nonprinting")
                            .short("v")
                            .long("show-nonprinting")
                            .help("use ^ and M- notation, except for LFD and TAB"))
                        .arg(Arg::with_name("INPUT")
                            .multiple(true)
                            .help("Sets the input file to use, when FILE is -, read standard input")
                            .index(1))
                        .get_matches();

    let show_all = matches.is_present("show-all");  // vET
    let e = matches.is_present("e");  // vE
    let t = matches.is_present("t");  // vT

    let print_options = PrintOptions {
        number_nonblank: matches.is_present("number-nonblank"),
        show_ends: matches.is_present("show-ends") || show_all || e,
        number_line: matches.is_present("number"),
        squeeze_blank: matches.is_present("squeeze-blank"),
        show_tabs: matches.is_present("show-tabs") || show_all || t,
        show_nonprinting: matches.is_present("show-nonprinting") || show_all || e || t
    };

//    println!("{:?}", print_options);

    if matches.is_present("INPUT") {
        let file_names: Vec<_> = matches.values_of("INPUT").unwrap().collect();
        for file_name in file_names {
            cat_file(file_name, &print_options);
        }
    } else {
        cat_stdin(&print_options);
    }
}

fn output_line(line: &str, line_number: usize, options: &PrintOptions) -> bool {
    if line.is_empty() && options.number_nonblank {
        println!("");
        return false;
    }

    if options.number_line {
        print!("{:>6}\t", line_number);
    }

    // TODO: deal with non-printable charactors
    if options.show_tabs {
        let new_line = line.replace("\t", "^I");
        print!("{}", new_line);
    } else {
        print!("{}", line);
    }

    if options.show_ends {
        print!("$")
    }

    println!("");
    return true;
}

fn cat_lines<T>(lines: std::io::Lines<T>, options: &PrintOptions) where T: std::io::BufRead {
    let mut line_number = 1;
    let mut prev_empty = false;

    for line in lines {
        match line {
            Ok(line) => {
                let is_empty = line.trim_right_matches("\n").is_empty();
                if !options.squeeze_blank || !is_empty || !prev_empty {
                    if output_line(&line, line_number, options) {
                        line_number += 1;
                    }
                }

                prev_empty = is_empty;
            }
            Err(err) => panic!("Error: {}", err),
        }
    }
}

fn cat_file(file_name: &str, options: &PrintOptions) {
    if file_name == "-" {
        cat_stdin(options);
        return;
    }

    let file = File::open(file_name).expect("File not found");
    cat_lines(BufReader::new(file).lines(), &options);
}

fn cat_stdin(options: &PrintOptions) {
    let stdin = std::io::stdin();
    cat_lines(stdin.lock().lines(), &options);
}