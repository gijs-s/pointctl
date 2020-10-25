use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};

fn main() {
    // let mut searched_neighborhoods = 0;
    let total_size = 10_015;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] Calculating neighbors [{bar:40.cyan/blue}] {pos}/{len} ({eta} left at {per_sec})")
        .progress_chars("#>-"));

    let _res: Vec<u64> = (0..total_size)
        .progress_with(pb)
        .map(|v| {
            thread::sleep(Duration::from_millis(12));
            v
        })
        .collect();
}
