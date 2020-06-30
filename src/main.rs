extern crate pointctl as pc;

use std::ffi::OsStr;
use std::path::Path;
use std::process::exit;

use clap::{crate_version, App, Arg, ArgMatches, SubCommand};

use pc::fs;
use pc::generate::generate_cube;
use pc::util::validator;

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
                        .validator(validator::is_integer)
                        .help("Amount of point used to generate"),
                )
                .arg(
                    Arg::with_name("OUTPUT")
                        .help("Sets the output file to use")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("reduce", Some(_sub_m)) => reduce_command(_sub_m),
        ("explain", Some(sub_m)) => explain_command(sub_m),
        ("view", Some(_sub_m)) => view_command(_sub_m),
        ("generate", Some(sub_m)) => generate_command(sub_m),
        (cmd, _) => println!("Could not parse options for `{}`", cmd),
    }
}

// Generate datasets
// pointclt generate 1000 ./output.csv
fn generate_command(matches: &ArgMatches) {
    // Find amount of points to generate from args, default to 10k
    let point_count: i32 = match matches.value_of("points") {
        None => {
            println!("Points where not specified, defaulting to 10000");
            10_000
        }
        Some(s) => match s.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Points argument was not a number");
                exit(1);
            }
        },
    };

    println!("Will generate {} points in cube pattern", point_count);

    // Find exactly to which file we will write and in which format.
    let output_file_path = matches.value_of("OUTPUT").unwrap();
    let output_file = Path::new(output_file_path);
    let file_extension = match output_file.extension().and_then(OsStr::to_str) {
        Some(v) => match v {
            "csv" => fs::SupportedFileFormat::CSV,
            "ply" => fs::SupportedFileFormat::PLY,
            e => {
                print!("Unsupported file format `{}` passed, defaulting to csv", e);
                fs::SupportedFileFormat::CSV
            }
        },
        None => {
            print!("No file format passed, defaulting to csv");
            fs::SupportedFileFormat::CSV
        }
    };

    // generate the points
    // TODO: Allow support for choosing different patterns here
    let generated_points = generate_cube(point_count, 0.05);
    println!("Generated {} points", generated_points.len());

    // Do a buffered write to file for all the points
    let res = match file_extension {
        fs::SupportedFileFormat::CSV => fs::csv::write(generated_points, output_file),
        fs::SupportedFileFormat::PLY => fs::ply::write(generated_points, output_file),
    };

    match res {
        Ok(_) => println!("All points written to file"),
        Err(e) => {
            eprintln!("Error when writing to file: {:?}", e);
            exit(1);
        },
    };
}

// Explain a dataset
fn explain_command(_matches: &ArgMatches) {
    // Retrieve the points from the original dataset
    // Retrieve the points from the reduced dataset
    // Zip these point into the Point format
    // Create a Da Silva explanation mechanism
    // Run the data through the mechanism and get a vector of annotated points back
    // Write these annotated points to file
    println!("Got some args");
}

// Command used to reduce datasets
fn reduce_command(_matches: &ArgMatches) {
    // Load in the nD dataset
    // Preform TSNE data reduction with the given arguments
    // Write the reduced 3D data to a new file
    println!("`reduce` not yet implemented")
}

fn view_command(_matches: &ArgMatches) {
    // Load in a custom file with annotated points
    // Determine which explanation was used, for now always only da silva
    // Pass the data to a viewing mechanism, which will
    //  - Create a color pallet for dimensions based on the global dimension randing
    //  - Render all the points with the given color
    // The viewing mechanism should allow basic navigation but need the ability for custom interactions in the future
    println!("`view` not yet implemented")
}
