extern crate pointctl as pc;

// Build in imports
use exp::Neighborhood;
use nalgebra::{Point2, Point3};
use std::path::Path;
use std::process::exit;

// Third party imports
use clap::{crate_version, App, Arg, ArgMatches, SubCommand};

// Local imports
use pc::fs::prelude::{read, write};
use pc::util::validator;
use pc::view::view::display;
use pc::{exp, generate};

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
                .about("Calculate a explanation given the original and reduced dataset (Only DaSilva right now)")
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
                    Arg::with_name("neighborhood_size_r")
                        .short("r")
                        .takes_value(true)
                        .validator(validator::is_float)
                        .help("The radius based P value used for the explanation process"),
                ).arg(
                    Arg::with_name("neighborhood_size_k")
                        .short("k")
                        .takes_value(true)
                        .validator(validator::is_usize)
                        .help("The count based P value used for the explanation process"),
                )
                .arg(
                    Arg::with_name("OUTPUT_FILE")
                    .short("o")
                    .help("Set the file to output the explained data to")
                    .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("view")
                .about(
                    "Allows you to view 3D data points given the original data, reduced points and \
                    the annotations. This command assumes that the reduced points, original data and \
                    annotations have matching indexes. The to start the viewer you need to provide the \
                    original data and either a 2d or 3d reduced set. Annotations are optional and can \
                    be computed from the viewer ")
                .arg(
                    Arg::with_name("original_data")
                        .short("i")
                        .long("input")
                        .required(true)
                        .takes_value(true)
                        .help("The original dataset in ply or csv format"),
                )
                .arg(
                    Arg::with_name("reduced_data_3d")
                        .short("r")
                        .long("r3d")
                        .takes_value(true)
                        .help("The 3D reduced dataset in ply or csv format"),
                )
                .arg(
                    Arg::with_name("annotations_3d")
                        .short("a")
                        .long("a3d")
                        .takes_value(true)
                        .help("Annotations for the 3D data in ply or csv format"),
                )
                .arg(
                    Arg::with_name("reduced_data_2d")
                        .short("x")
                        .long("r2d")
                        .takes_value(true)
                        .help("The 2D reduced dataset in ply or csv format"),
                )
                .arg(
                    Arg::with_name("annotations_2d")
                        .short("b")
                        .long("a2d")
                        .takes_value(true)
                        .help("Annotations for the 2D data in ply or csv format")
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
                    Arg::with_name("shape")
                        .default_value("cube")
                        .short("s")
                        .long("shape")
                        .takes_value(true)
                        .help("The type of shape you wish to create, chose from `cube` and `hypercube`"),
                )
                .arg(
                    Arg::with_name("OUTPUT_FILE")
                        .required(true)
                        .index(1)
                        .help("Sets the output file to use"),
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

    enum Shapes {
        Cube,
        HyperCube,
    }

    // Find out which pattern should be used
    let pattern = match matches.value_of("shape") {
        Some("cube") => Shapes::Cube,
        Some("hypercube") => Shapes::HyperCube,
        Some(v) => {
            eprint!("Invalid value was passed as shape, received `{}`", v);
            exit(17)
        }
        _ => panic!("This should not happend, programming error"),
    };

    // Retrieve the output file from args, unwrap is safe since output file is required.
    let output_file_path = matches.value_of("OUTPUT_FILE").unwrap();
    let output_file = Path::new(output_file_path);

    // generate the points
    let generated_points = match pattern {
        Shapes::Cube => {
            println!("Will generate {} points in cube pattern", point_count);
            generate::generate_cube(point_count, 0.00)
        }
        Shapes::HyperCube => {
            println!("Will generate {} points in hypercube pattern", point_count);
            generate::generate_hyper_cube(point_count, 0.00)
        }
    };

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
            [x, y, z] => Point3::<f32>::new(x, y, z),
            [x, y] => Point3::<f32>::new(x, y, 0.0),
            _ => {
                eprint!("Points with {} dimensions is not supported yet", vec.len());
                exit(15)
            }
        })
        .collect::<Vec<Point3<f32>>>();

    let neigborhoods_size = match (
        matches.value_of("neighborhood_size_r"),
        matches.value_of("neighborhood_size_k"),
    ) {
        (None, None) => {
            eprint!("No neighborhood size was choses");
            exit(17);
        }
        // Safe because this was already checked by the validator in the CLI
        (Some(r_str), None) => Neighborhood::R(r_str.parse::<f32>().unwrap()),
        (None, Some(k_str)) => Neighborhood::K(k_str.parse::<usize>().unwrap()),
        (Some(_), Some(_)) => {
            eprint!("Two types of neighborhood size were chosen, please select only one");
            exit(17);
        }
    };

    // Create a Da Silva explanation mechanism
    let da_silva_mechanism =
        exp::da_silva::DaSilvaMechanismState::new(clean_reduced_points, &original_points);
    let da_silva_explanation = da_silva_mechanism.explain(neigborhoods_size, None);

    // Write the annotations to file
    let annotations = da_silva_explanation
        .iter()
        .map(|exp| vec![exp.attribute_index as f32, exp.confidence])
        .collect();
    let output_file_path = matches.value_of("OUTPUT_FILE").unwrap();
    let output_file = Path::new(output_file_path);

    // Write these annotated points to file
    write(output_file, annotations);
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
    let reduced_points_3d = match matches.value_of("reduced_data_3d") {
        None => None,
        Some(reduced_data_path) => {
            let (reduced_data, r) = read(Path::new(reduced_data_path));
            println!(
                "Reduced 3D data loaded. Consists of {} points with {} dimensions",
                reduced_data.len(),
                r
            );

            // Convert reduced data to 3D nalgebra points with optional zero padding
            // TODO create abstraction for this!
            let reduced_points = reduced_data
                .iter()
                .map(|vec| match vec[..] {
                    [x, y, z] => Point3::<f32>::new(x, y, z),
                    [x, y] => Point3::<f32>::new(x, y, 0.0),
                    _ => {
                        eprint!("Points with {} dimensions is not supported yet", vec.len());
                        exit(15)
                    }
                })
                .collect::<Vec<Point3<f32>>>();
            Some(reduced_points)
        }
    };

    // Retrieve the points from the reduced dataset
    let explanations_3d = match matches.value_of("annotations_3d") {
        None => None,
        Some(annotations_path) => {
            let annotations_data = Path::new(annotations_path);
            let (annotations, d) = read(annotations_data);
            println!(
                "Da Silva annotations for 3D loaded. Consists of {} points with {} dimensions",
                annotations.len(),
                d
            );

            let res = annotations
                .iter()
                .map(|v| exp::da_silva::DaSilvaExplanation {
                    attribute_index: v[0] as usize,
                    confidence: v[1],
                })
                .collect::<Vec<exp::da_silva::DaSilvaExplanation>>();
            Some(res)
        }
    };

    // Parse the 2D points if the reduced data is provided
    let reduced_points_2d: Option<Vec<Point2<f32>>> = {
        if let Some(path) = matches.value_of("reduced_data_2d") {
            let (reduced_data, d) = read(Path::new(path));
            println!(
                "Reduced 2D data loaded. Consists of {} points with {} dimensions",
                reduced_data.len(),
                d
            );
            let reduced_points = reduced_data
                .iter()
                .map(|vec| match vec[..] {
                    [x, y] => Point2::new(x, y),
                    _ => {
                        eprint!("Points with {} dimensions is not supported yet", vec.len());
                        exit(15)
                    }
                })
                .collect::<Vec<Point2<f32>>>();
            Some(reduced_points)
        } else {
            None
        }
    };

    let explanations_2d: Option<Vec<exp::da_silva::DaSilvaExplanation>> = {
        if let Some(path) = matches.value_of("annotations_2d") {
            let (annotations, d) = read(Path::new(path));
            println!(
                "Da Silva annotations for 2D loaded. Consists of {} points with {} dimensions",
                annotations.len(),
                d
            );
            let explanations = annotations
                .iter()
                .map(|v| exp::da_silva::DaSilvaExplanation {
                    attribute_index: v[0] as usize,
                    confidence: v[1],
                })
                .collect::<Vec<exp::da_silva::DaSilvaExplanation>>();
            Some(explanations)
        } else {
            None
        }
    };

    display(
        original_points,
        reduced_points_2d,
        explanations_2d,
        reduced_points_3d,
        explanations_3d,
    );
}
