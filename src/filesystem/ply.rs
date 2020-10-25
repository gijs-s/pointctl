//! A small helper for reading and writing PLY files.
//
// TODO: use nom to create a parser for the files.

use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

use crate::util::types::PointN;

/// Write all points to a simple ply file
pub fn write(file_path: &Path, points: Vec<PointN>) -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create(file_path)?);

    // Very basic simple header, assumes the data is 3D
    let header = format!("ply\nformat ascii 1.0\ncomment Ply file generated using pointclt processing tool\ncomment Tool created by Gijs van Steenpaal\nelement point {}\nproperty float x\nproperty float y\nproperty float z\nend_header\n", points.len());
    buffer.write_all(header.as_bytes())?;

    // For all the generated points write them to file.
    for (i, p) in points.iter().enumerate() {
        // Generate a single line from a point
        let strings: Vec<String> = p.iter().map(|n| n.to_string()).collect();
        writeln!(buffer, "{}", strings.join(", "))?;

        // Once every milion points we flush the data to disk.
        if i % 1_000_000 == 0 {
            buffer.flush()?;
        };
    }
    Ok(())
}

/// Retrieve the header line of a file, can be used to check the dimension count
pub fn get_header(_file_path: &Path) -> std::io::Result<Vec<String>> {
    unimplemented!("Reading from PLY is not yet supported")
}

/// Read a CSV file from disk
/// TODO: Build this using NOM
pub fn read(_file_path: &Path) -> std::io::Result<(Vec<PointN>, usize, Vec<String>)> {
    unimplemented!("Reading from PLY is not yet supported")
}
