//! Main interface with file system.

use std::ffi::OsStr;
use std::path::Path;
use std::process::exit;

use super::{csv, ply};
use crate::util::types::PointN;

// Enum for supported file formats
#[derive(Debug, PartialEq)]
pub enum SupportedFileFormat {
    CSV,
    PLY,
}

/// Generic function to write a collection of points to disk.
/// File format will be decided upon by extension, defaulting to CSV.
pub fn write(file_path: &Path, points: Vec<PointN>) {
    let res = match get_file_extension(file_path) {
        SupportedFileFormat::CSV => csv::write(file_path, points),
        SupportedFileFormat::PLY => ply::write(file_path, points),
    };

    match res {
        Ok(_) => println!("All points written to file"),
        Err(e) => {
            eprintln!("Error when writing to file: {:?}", e);
            exit(31);
        }
    };
}

/// Retrieve the header line of a file, can be used to check the dimension count
pub fn get_header(file_path: &Path) -> Vec<String> {
    let res = match get_file_extension(file_path) {
        SupportedFileFormat::CSV => csv::get_header(file_path),
        SupportedFileFormat::PLY => ply::get_header(file_path),
    };
    match res {
        Ok(data) => data,
        Err(e) => {
            eprintln!("{:?}", e);
            exit(12)
        }
    }
}

/// Generic function to read points from disk.
/// Will return the points and their dimensionality.
pub fn read(file_path: &Path) -> (Vec<PointN>, usize, Vec<String>) {
    let res = match get_file_extension(file_path) {
        SupportedFileFormat::CSV => csv::read(file_path),
        SupportedFileFormat::PLY => ply::read(file_path),
    };

    match res {
        Ok(data) => data,
        Err(e) => {
            eprintln!("There was an IO error reading from file: {:?}", e);
            exit(10)
        }
    }
}

fn get_file_extension(file_path: &Path) -> SupportedFileFormat {
    match file_path.extension().and_then(OsStr::to_str) {
        Some(v) => match v {
            "csv" => SupportedFileFormat::CSV,
            "ply" => SupportedFileFormat::PLY,
            alt => {
                print!(
                    "Unsupported file format `{}` passed, writing to file in CSV format",
                    alt
                );
                SupportedFileFormat::CSV
            }
        },
        None => {
            print!("No file format passed, writing to file in CSV format");
            SupportedFileFormat::CSV
        }
    }
}
