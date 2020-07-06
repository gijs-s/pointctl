extern crate pointctl as pc;

// Build in imports
use std::path::Path;
use std::process::exit;

// Third party imports
use clap::{crate_version, App, Arg, ArgMatches, SubCommand};

// Local imports
use pc::exp;
use pc::fs::prelude::{read, write};
use pc::generate::generate_cube;
use pc::view::view::display;
use pc::util::{types::Point3, validator};

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
                .alias("exp")
                .about("Calculate a explanation given the original and reduced dataset (2D only for now)")
                .arg(
                    Arg::with_name("original_data")
                        .short("i")
                        .required(true)
                        .takes_value(true)
                        .help("The original dataset in ply or csv format"),
                )
                .arg(
                    Arg::with_name("reduced_data")
                        .short("r")
                        .required(true)
                        .takes_value(true)
                        .help("The reduced dataset in ply or csv format"),
                ).arg(
                    Arg::with_name("OUTPUT_FILE")
                    .short("o")
                    .help("Set the file to output the explained data to")
                    .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("view")
                .about("View a set of possibly annotated points")
                .arg(
                    Arg::with_name("reduced_data")
                        .short("i")
                        .required(true)
                        .takes_value(true)
                        .help("The reduced dataset in ply or csv format"),
                )
                .arg(
                    Arg::with_name("annotations")
                        .short("a")
                        .required(true)
                        .takes_value(true)
                        .help("Collection of annotations in ply or csv format"),
                ),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .alias("gen")
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
                    Arg::with_name("OUTPUT_FILE")
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
// TODO: support for noise
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

    // Retrieve the output file from args, unwrap is safe since output file is required.
    let output_file_path = matches.value_of("OUTPUT_FILE").unwrap();
    let output_file = Path::new(output_file_path);

    // generate the points
    // TODO: Allow support for choosing different patterns here
    let generated_points = generate_cube(point_count, 0.00);
    println!("Generated {} points", generated_points.len());

    // Do a buffered write to file for all the points
    write(output_file, generated_points);
}

// Explain a dataset
fn explain_command(matches: &ArgMatches) {
    // Retrieve the points from the original dataset
    let original_data_path = matches.value_of("original_data").unwrap();
    let original_data = Path::new(original_data_path);
    let (original_points, n) = read(original_data);
    println!(
        "Original data loaded. Consists of {} points with {} dimensions",
        original_points.len(),
        n
    );

    // Retrieve the points from the reduced dataset
    let reduced_data_path = matches.value_of("reduced_data").unwrap();
    let reduced_data = Path::new(reduced_data_path);
    let (reduced_points, r) = read(reduced_data);
    println!(
        "Reduced data loaded. Consists of {} points with {} dimensions",
        reduced_points.len(),
        r
    );

    // Convert reduced data to 3D nalgebra points with optional zero padding
    let clean_reduced_points = reduced_points
        .iter()
        .map(|vec| match vec[..] {
            [x, y, z] => Point3::new(x, y, z),
            [x, y] => Point3::new(x, y, 0.0),
            _ => {
                eprint!("Points with {} dimensions is not supported yet", vec.len());
                exit(15)
            }
        })
        .collect::<Vec<Point3>>();

    // Zip these two sets of points into the the Point struct format. Drop original data out of scope after.
    let points = original_points
        .iter()
        .zip(clean_reduced_points)
        .map(|(o, r)| exp::common::Point {
            reduced: r,
            original: o.to_owned(),
        })
        .collect::<Vec<exp::common::Point>>();

    println!("{} Points structures created", points.len());

    // Create a Da Silva explanation mechanism
    // TODO: Normalize the TSNE projection or use relative neighborhood size.
    let (da_silva_explanation, _dimension_ranking) = exp::da_silva::explain(&points, 0.1);

    // TODO: Temp write everything to file.
    // // Does not give a lot of insight though.
    // let ap = points
    //     .iter()
    //     .map(|p| &p.reduced)
    //     .zip(da_silva_explanation)
    //     .map(|(p, a)| {
    //         vec![p.x, p.y, p.z, a.attribute_index as f32, a.confidence]
    //     })
    //     .collect();

    // Write the annotations to file
    let annotations = da_silva_explanation.iter().map(|exp| vec![exp.attribute_index as f32, exp.confidence]).collect();
    let output_file_path = matches.value_of("OUTPUT_FILE").unwrap();
    let output_file = Path::new(output_file_path);
    write(output_file, annotations);

    // Run the data through the mechanism and get a vector of annotated points back
    // Write these annotated points to file
}

// Command used to reduce datasets
fn reduce_command(_matches: &ArgMatches) {
    // Load in the nD dataset
    // Preform TSNE data reduction with the given arguments
    // Write the reduced 3D data to a new file
    println!("`reduce` not yet implemented. You can use python's SciKit learn for now")
}

fn view_command(matches: &ArgMatches) {
    // Load in a custom file with annotated points
    // Determine which explanation was used, for now always only da silva
    // Pass the data to a viewing mechanism, which will
    //  - Create a color pallet for dimensions based on the global dimension randing
    //  - Render all the points with the given color
    // The viewing mechanism should allow basic navigation but need the ability for custom interactions in the future


    // Retrieve the points from the reduced dataset
    let reduced_data_path = matches.value_of("reduced_data").unwrap();
    let reduced_data = Path::new(reduced_data_path);
    let (reduced_points, r) = read(reduced_data);
    println!(
        "Reduced data loaded. Consists of {} points with {} dimensions",
        reduced_points.len(),
        r
    );

    // Convert reduced data to 3D nalgebra points with optional zero padding
    // TODO create abstraction for this!
    let clean_reduced_points = reduced_points
    .iter()
    .map(|vec| match vec[..] {
        [x, y, z] => Point3::new(x, y, z),
        [x, y] => Point3::new(x, y, 0.0),
        _ => {
            eprint!("Points with {} dimensions is not supported yet", vec.len());
            exit(15)
        }
    })
    .collect::<Vec<Point3>>();


    // Retrieve the points from the reduced dataset
    let annotations_path = matches.value_of("annotations").unwrap();
    let annotations_data = Path::new(annotations_path);
    let (annotations, d) = read(annotations_data);
    println!(
        "Annotations loaded. Consists of {} points with {} dimensions",
        annotations.len(),
        d
    );

    let da_silva_explanations = annotations.iter().map(|v| exp::da_silva::DaSilvaExplanation {
        attribute_index: v[0] as usize, confidence: v[1]
    }).collect();

    display("Da silva explanation", clean_reduced_points, da_silva_explanations);
}
