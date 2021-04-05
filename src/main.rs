extern crate nalgebra as na;
extern crate pointctl as pc;

// Build in imports
use exp::Neighborhood;
use pc::search::{PointContainer2D, PointContainer3D};
use std::{convert::TryFrom, path::Path, process::exit};

// Third party imports
use clap::{crate_version, App, AppSettings, Arg, ArgMatches, SubCommand};

// Local imports
use pc::{
    exp,
    filesystem::{get_header, write},
    generate,
    util::validator,
    view,
};

fn main() {
    // TODO: Move this entire mess to a yaml file or use the nice makro. See https://docs.rs/clap/2.33.1/clap/
    let matches = App::new("Point cloud processing")
        .version(crate_version!())
        .author("Gijs van Steenpaal <g.j.vansteenpaal@students.uu.nl>")
        .about("Program for generating, processing and explaining point clouds")
        .subcommand(
            SubCommand::with_name("explain")
                .alias("exp")
                .about("Calculate a explanation given the original and reduced dataset. \
                This will output the data to a simple csv file with 2 values on each row, \
                the first being dimensionality/dimension and second the confidence. Both \
                the input and output csv files will use ; as delimiter.")
                .arg(
                    Arg::with_name("original_data")
                        .short("i")
                        .long("input")
                        .required(true)
                        .takes_value(true)
                        .help("The original dataset in csv format (; as delimiter)"),
                )
                .arg(
                    Arg::with_name("reduced_data")
                        .short("d")
                        .long("reduced")
                        .required(true)
                        .takes_value(true)
                        .help("The reduced dataset in csv format (; as delimiter)"),
                ).arg(
                    Arg::with_name("mechanism")
                        .short("m")
                        .long("mechanism")
                        .default_value("silva_variance")
                        .takes_value(true)
                        // TODO: Add more possible validation techniques
                        .possible_values(&["silva_variance", "silva_euclidean", "driel_min", "driel_sum"])
                        .help("Chose which annotation technique is used, da silva's variance and distance\
                            based explanation or van Driel PCA metric.")
                )
                .arg(
                    Arg::with_name("theta")
                        .short("t")
                        .long("theta")
                        .required_ifs(&[("mechanism", "driel_min"),("mechanism", "driel_total")])
                        .takes_value(true)
                        .validator(validator::is_norm_float)
                )
                .arg(
                    Arg::with_name("neighborhood_size_r")
                        .short("r")
                        .long("radius")
                        .conflicts_with("neighborhood_size_k")
                        .required_unless("neighborhood_size_k")
                        .takes_value(true)
                        .validator(validator::is_norm_float)
                        .help("The radius based P value used for the explanation process"),
                ).arg(
                    Arg::with_name("neighborhood_size_k")
                        .short("k")
                        .long("neighbor_count")
                        // .conflicts_with("neighborhood_size_r")
                        .required_unless("neighborhood_size_r")
                        .takes_value(true)
                        .validator(validator::is_usize)
                        .help("The count based P value used for the explanation process"),
                )
                .arg(
                    Arg::with_name("jobs")
                        .short("j")
                        .long("jobs")
                        .takes_value(true)
                        .validator(validator::is_usize)
                        .help("The number of threads the program can use, by default it uses all")
                )
                .arg(
                    Arg::with_name("OUTPUT_FILE")
                    .index(1)
                    .required(true)
                    .help("Set the file to output the explained data to (ply or csv format)")
                ),
        )
        .subcommand(
            SubCommand::with_name("view")
                .about(
                    "Allows you to view 3D data points given the original data and reduced points. This \
                    command assumes that the reduced points, original data have matching indexes. \
                    With the viewer running you can run the explanation algorithms and view them directly. \
                    To start the viewer you need to provide the original data combined the 2d and/or 3d reduced set.")
                .arg(
                    Arg::with_name("original_data")
                        .short("i")
                        .long("input")
                        .required(true)
                        .takes_value(true)
                        .help("The original dataset in csv format (; as delimiter)"),
                )
                .arg(
                    Arg::with_name("reduced_data_2d")
                        .short("x")
                        .long("r2d")
                        .required_unless("reduced_data_3d")
                        .takes_value(true)
                        .help("The 2D reduced dataset in csv format (; as delimiter)"),
                )
                .arg(
                    Arg::with_name("reduced_data_3d")
                        .short("r")
                        .long("r3d")
                        .required_unless("reduced_data_2d")
                        .takes_value(true)
                        .help("The 3D reduced dataset in csv format (; as delimiter)"),
                )
                .arg(
                    Arg::with_name("jobs")
                        .short("j")
                        .long("jobs")
                        .takes_value(true)
                        .validator(validator::is_usize)
                        .help("The number of threads the program can use, by default it uses all")
                )
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
                        .short("s")
                        .long("shape")
                        .default_value("cube")
                        .takes_value(true)
                        .help("The type of shape you wish to create, chose from `cube` and `hypercube`"),
                )
                .arg(
                    Arg::with_name("noise")
                        .short("n")
                        .long("noise")
                        .default_value("0.0")
                        .takes_value(true)
                        .validator(validator::is_float)
                        .help("The amount of noise to introduce to each point."),
                )
                .arg(
                    Arg::with_name("OUTPUT_FILE")
                        .index(1)
                        .required(true)
                        .help("Sets the output file to use"),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
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
        _ => panic!("This should not happen, result of a programming error"),
    };

    let noise = match matches.value_of("noise") {
        None => 0.0f32,
        Some(n_str) => {
            let n = n_str.parse::<f32>().unwrap();
            if n < 0.0f32 {
                0.0f32
            } else {
                n
            }
        }
    };

    // generate the points
    let generated_points = match pattern {
        Shapes::Cube => {
            println!("Will generate {} points in cube pattern", point_count);
            generate::generate_cube(point_count, noise)
        }
        Shapes::HyperCube => {
            println!("Will generate {} points in hypercube pattern", point_count);
            generate::generate_hyper_cube(point_count, noise)
        }
    };

    println!("Generated {} points", generated_points.len());

    // Retrieve the output file from args, unwrap is safe since output file is required.
    let output_file_path = matches.value_of("OUTPUT_FILE").unwrap();
    let output_file = Path::new(output_file_path);
    // Do a buffered write to file for all the points
    write(output_file, generated_points);
}

// Explain a dataset
fn explain_command(matches: &ArgMatches) {
    // Check how many threads should be used
    if let Some(value) = matches.value_of("jobs") {
        // Unwrap is safe because of the validator
        let j = value.parse::<usize>().unwrap();
        match rayon::ThreadPoolBuilder::new()
            .num_threads(j)
            .build_global()
        {
            Ok(_) => println!("Using {} threads when computing the metrics", j),
            Err(e) => {
                eprintln!("Could not set thread count because\n {:?}", e);
                exit(42)
            }
        }
    }

    // Find out which mechanism was used. Unwrap is safe since value
    // has a default and a validator
    let explanation_mode: view::ExplanationMode = {
        let mechanism_text = matches.value_of("mechanism").unwrap();
        view::ExplanationMode::try_from(mechanism_text).unwrap()
    };

    // Find out what the dimensionality is of the reduced data
    let reduced_data_path: &Path = {
        let path_str = matches.value_of("reduced_data").unwrap();
        Path::new(path_str)
    };
    let dimensionality = get_header(reduced_data_path).len();

    // Construct the path for the original points
    let original_data_path: &Path = {
        let path_str = matches.value_of("original_data").unwrap();
        Path::new(path_str)
    };

    // Construct the path for the output file
    let output_file_path: &Path = {
        let path_str = matches.value_of("OUTPUT_FILE").unwrap();
        Path::new(path_str)
    };

    // Parse the choses neighborhood size
    let neighborhoods_size: Neighborhood = match (
        matches.value_of("neighborhood_size_r"),
        matches.value_of("neighborhood_size_k"),
    ) {
        // Safe because this was already checked by the validator in the CLI.
        // Clap also ensures exactly one of the 2 is chosen
        (Some(r_str), None) => Neighborhood::R(r_str.parse::<f32>().unwrap()),
        (None, Some(k_str)) => Neighborhood::K(k_str.parse::<usize>().unwrap()),
        (_, _) => panic!("Impossible neighborhood combination passed, if this is raised the constraints set in the CLI are broken")
    };

    // if the van driel explanation is used the theta is mandatory in the cli.
    // We already run a validator to ensure it is between 1 and 0 and that it
    // is present iff driel is selected.
    let theta: Option<f32> = matches
        .value_of("theta")
        .and_then(|v| Some(v.parse::<f32>().unwrap()));

    match (dimensionality, explanation_mode) {
        (2, view::ExplanationMode::DaSilva(method)) => {
            let point_container = PointContainer2D::new(original_data_path, reduced_data_path);
            let annotations: Vec<exp::DaSilvaExplanation> =
                exp::run_da_silva_2d(&point_container, neighborhoods_size, method);
            let points = annotations
                .iter()
                .map(|exp| vec![exp.attribute_index as f32, exp.confidence])
                .collect();
            write(&output_file_path, points);
        }
        (3, view::ExplanationMode::DaSilva(method)) => {
            let point_container = PointContainer3D::new(original_data_path, reduced_data_path);
            let annotations: Vec<exp::DaSilvaExplanation> =
                exp::run_da_silva_3d(&point_container, neighborhoods_size, method);
            let points = annotations
                .iter()
                .map(|exp| vec![exp.attribute_index as f32, exp.confidence])
                .collect();
            write(&output_file_path, points);
        }
        (2, view::ExplanationMode::VanDriel(method)) => {
            let point_container = PointContainer2D::new(original_data_path, reduced_data_path);
            let annotations: Vec<exp::VanDrielExplanation> =
                exp::run_van_driel_2d(&point_container, neighborhoods_size, theta.unwrap(), method);
            let points = annotations
                .iter()
                .map(|exp| vec![exp.dimension as f32, exp.confidence])
                .collect();
            write(&output_file_path, points);
        }
        (3, view::ExplanationMode::VanDriel(method)) => {
            let point_container = PointContainer3D::new(original_data_path, reduced_data_path);
            let annotations: Vec<exp::VanDrielExplanation> =
                exp::run_van_driel_3d(&point_container, neighborhoods_size, theta.unwrap(), method);
            let points = annotations
                .iter()
                .map(|exp| vec![exp.dimension as f32, exp.confidence])
                .collect();
            write(&output_file_path, points);
        }
        (v, _) => {
            eprintln!(
                "The amount of dimensions in the header of the reduced file \
            indicated that there were '{}' dimensions, this is not yet supported.",
                v
            );
            exit(15)
        }
    };
}

// Command used to reduce datasets
fn reduce_command(_matches: &ArgMatches) {
    // Load in the nD dataset
    // Preform TSNE data reduction with the given arguments
    // Write the reduced data to a new file
    println!(
        "`reduce` not yet implemented. You can use python's SciKit learn for now in ./python-repl"
    )
}

fn view_command(matches: &ArgMatches) {
    // Load in a custom files with all the points
    // Pass the data to a viewing mechanism, which will
    //  - Create a color pallet for dimensions based on the global dimension randing
    //  - Render all the points with the given color
    // TODO: Add robust way to pass the precomputed annotations

    // Check how many threads should be used
    if let Some(value) = matches.value_of("jobs") {
        // Unwrap is safe because of the validator
        let j = value.parse::<usize>().unwrap();
        match rayon::ThreadPoolBuilder::new()
            .num_threads(j)
            .build_global()
        {
            Ok(_) => println!("Using {} threads to compute the metric", j),
            Err(e) => {
                eprintln!("Could not set thread count because\n {:?}", e);
                exit(42)
            }
        }
    }

    // Retrieve the points from the original dataset
    let original_data_path = {
        let path_str = matches.value_of("original_data").unwrap();
        Path::new(path_str)
    };

    // If a path to the 3D points was given we load it into the point container
    let point_container_3d = matches
        .value_of("reduced_data_3d")
        .and_then(|reduced_data_path| {
            let reduced_path = Path::new(reduced_data_path);
            Some(PointContainer3D::new(original_data_path, reduced_path))
        });

    // If a path to the 2D points was given we load it into the point container
    let point_container_2d = matches
        .value_of("reduced_data_2d")
        .and_then(|reduced_data_path| {
            let reduced_path = Path::new(reduced_data_path);
            Some(PointContainer2D::new(original_data_path, reduced_path))
        });

    view::display(point_container_2d, point_container_3d);
}
