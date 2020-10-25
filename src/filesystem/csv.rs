//! A small helper for reading and writing CSV files without headers
//
// TODO: use nom instead of of manual stuff.

use std::fs::File;
use std::io::{prelude::*, BufReader, BufWriter};
use std::path::Path;
use std::process::exit;

use crate::util::types::PointN;

pub fn write(file_path: &Path, points: Vec<PointN>) -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create(file_path)?);

    // File does not have a header
    // let header = format!();
    // buffer.write_all(header.as_bytes())?;
    writeln!(buffer, "x;y;z")?;

    // For all the generated points write them to file.
    for (i, p) in points.iter().enumerate() {
        // Generate a single line from a point
        let strings: Vec<String> = p.iter().map(|n| n.to_string()).collect();
        writeln!(buffer, "{}", strings.join(";"))?;

        // Once every milion points we flush the data to disk.
        if i % 1_000_000 == 0 {
            buffer.flush()?;
        };
    }
    Ok(())
}

/// Retrieve the header line of a file, can be used to check the dimension count
pub fn get_header(file_path: &Path) -> std::io::Result<Vec<String>> {
    let buffer = BufReader::new(File::open(file_path)?);

    match buffer.lines().next() {
        None => {
            eprintln!("File passed was empty");
            exit(12)
        }
        Some(res) => match res {
            Ok(line) => {
                let data = line
                    .split(';')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                Ok(data)
            }
            Err(e) => {
                eprintln!("Error reading line from csv: {:?}", e);
                exit(12)
            }
        },
    }
}

// Read a CSV file from disk
pub fn read(file_path: &Path) -> std::io::Result<(Vec<PointN>, usize, Vec<String>)> {
    let buffer = BufReader::new(File::open(file_path)?);

    let mut raw_lines = buffer.lines();

    let header = match raw_lines.next() {
        None => {
            eprintln!("File passed was empty");
            exit(12)
        }
        Some(res) => match res {
            Ok(line) => line
                .split(';')
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            Err(e) => {
                eprintln!("Error reading first line from csv: {:?}", e);
                exit(12)
            }
        },
    };

    let points = raw_lines
        .map(|line| match line {
            // Parse header line
            Ok(line) => {
                // We read a line from the file, split on `;` and attempt to parse each float.
                match line
                    .split(';')
                    .map(|s| s.trim().parse::<f32>())
                    .collect::<Result<Vec<_>, _>>()
                {
                    // All floats parsed successfully, return the point
                    Ok(point) => point,
                    Err(e) => {
                        eprintln!(
                            "Error parsing float in csv file: `{:?}`. Affected line:\n{}",
                            e, line
                        );
                        exit(11)
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading line from csv: {:?}", e);
                exit(12)
            }
        })
        .collect::<Vec<PointN>>();

    let length = match points.first() {
        Some(vec) => vec.len(),
        None => {
            eprintln!("CSV file was empty");
            exit(13)
        }
    };

    if points.iter().any(|p| length != p.len()) {
        eprintln!("Not all points have the same dimensionality");
        exit(14);
    }

    Ok((points, length, header))
}

// TODO: Add tests for reading data using io::Cursor
