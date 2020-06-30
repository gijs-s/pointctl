use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::Path;

pub fn write(points: Vec<Vec<f32>>, file_path: &Path) -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create(file_path)?);

    // File does not have a header
    // let header = format!();
    // buffer.write_all(header.as_bytes())?;

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

pub fn read(_file_path: &str) -> std::io::Result<Vec<Vec<f32>>> {
    println!("Reading as fast as i can, seriously i try");
    Ok(Vec::new())
}