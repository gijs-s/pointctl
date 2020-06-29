extern crate pointctl as pc;

use clap::{crate_version, App, Arg, SubCommand};

use pc::fs::ply::write;
use pc::generate::generate_cube;

fn main() {
    // TODO: Move this entire mess to a yaml file. See https://docs.rs/clap/2.33.1/clap/
    let matches = App::new("Point cloud processing")
        .version(crate_version!())
        .author("Gijs van Steenpaal <g.j.vansteenpaal@students.uu.nl>")
        .about("Program for generating, processing and explaining point clouds")
        .subcommand(
            SubCommand::with_name("reduce").about("Reduce a nD dataset to 2D or 3D"),
        )
        .subcommand(
            SubCommand::with_name("explain")
                .about("Calculate a explanation given the original and reduced dataset (2D only for now)")
                .arg(
                    Arg::with_name("original_data")
                        .short("i")
                        .required(true)
                        .help("The original dataset in ply or csv format"),
                )
                .arg(
                    Arg::with_name("reduced_data")
                        .short("r")
                        .required(true)
                        .help("The reduced dataset in ply or csv format"),
                ).arg(
                    Arg::with_name("output_image")
                    .short("o")
                    .help("The image to output too, if absent the images will just be shown."),
                ),
        )
        .subcommand(
            SubCommand::with_name("view")
                .about("View a set of possibly annotated points")
                .arg(
                    Arg::with_name("reduced_data")
                        .short("i")
                        .required(true)
                        .help("The reduced dataset in ply or csv format"),
                )
                .arg(
                    Arg::with_name("annotations")
                        .short("a")
                        .required(true)
                        .help("Collection of annotations in ply or csv format"),
                ),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about("Generate synthetic point clouds")
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
            SubCommand::with_name("modify")
                .about("Modify a point cloud quickly. Used to scale, move and rotate points."),
        )
        .get_matches();

    // TODO: Move the matching of each subcommand into their own functions.
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

        // TODO: make this optionally write to a csv instead.
        match write(res, file) {
            Ok(_) => println!("All points written to file"),
            Err(e) => println!("Error when writing to file: {:?}", e),
        };
    }
}
