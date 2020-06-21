use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

/// Write all points to a simple ply file
pub fn write(points: Vec<Vec<f32>>, file_path: &str) -> std::io::Result<()> {
    let mut buffer = BufWriter::new(File::create(file_path)?);
    let header = format!(
        "ply
format ascii 1.0
comment Ply file generated using rust pointcloud-processing tool
comment Created by Gijs van Steenpaal
element point {}
property float x
property float y
property float z
end_header
",
        points.len()
    );

    buffer.write_all(header.as_bytes())?;

    for (i, p) in points.iter().enumerate() {
        let strings: Vec<String> = p.iter().map(|n| n.to_string()).collect();
        writeln!(buffer, "{}", strings.join(", "))?;
        if i % 1000000 == 0 {
            buffer.flush()?;
        };
    }
    Ok(())
}
