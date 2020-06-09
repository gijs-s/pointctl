extern crate pointcloud_processing as pc;

use clap::{crate_version, App, Arg, SubCommand};

use pc::generate::generate_cube;
use pc::ply::write;

fn main() {
    let matches = App::new("Point cloud processing")
        .version(crate_version!())
        .author("Gijs van Steenpaal <g.j.vansteenpaal@students.uu.nl>")
        .about("Program for reading and processing point clouds")
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate synthetic point clouds")
                .author("Gijs van Steenpaal <g.j.vansteenpaal@students.uu.nl>")
                .arg(
                    Arg::with_name("points")
                        .short("p")
                        .long("points")
                        .required(true)
                        .takes_value(true)
                        .help("Amount of point used to generate"),
                )
                .arg(
                    Arg::with_name("OUTPUT")
                        .help("Sets the output file to use")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("controls testing features")
                .author("Gijs van Steenpaal <g.j.vansteenpaal@students.uu.nl>")
                .arg(
                    Arg::with_name("debug")
                        .short("d")
                        .help("print debug information verbosely"),
                ),
        )
        .get_matches();

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    // println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

    if let Some(matches) = matches.subcommand_matches("generate") {
        let points: i32 = match matches.value_of("points") {
            None => {
                println!("Points where not specified, defaulting to 10000");
                10_000
            }
            Some(s) => match s.parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    println!("Points argument was not a number, defaulting to 10000");
                    10_000
                }
            },
        };

        let file = matches.value_of("OUTPUT").unwrap();

        println!("Will generate {} points", points);
        let res = generate_cube(points, 0.05);
        println!("Generated {} points", res.len());

        match write(res, file) {
            Ok(_) => println!("All points written to file"),
            Err(e) => println!("Error when writing to file: {:?}", e)
        };

    }
}
