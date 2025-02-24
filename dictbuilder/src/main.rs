use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    // Get the file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];

    // Read the file line by line
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();
    let mut unique_lines: HashSet<String> = HashSet::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if index < 4 {
            // Keep the first 4 lines as they are
            lines.push(line.clone());
        } else {
            // For lines from 5 onwards, check for duplicates
            if unique_lines.insert(line.clone()) {
                lines.push(line);
            }
        }
    }

    // Sort the lines from line 5 onwards
    let mut sorted_part = lines[4..].to_vec();
    sorted_part.sort();

    // Combine the first 4 lines with the sorted lines
    let final_lines: Vec<String> = lines[..4].to_vec().into_iter().chain(sorted_part).collect();

    // Write the result back to the file
    let mut output_file = File::create(file_path)?;
    for line in final_lines {
        writeln!(output_file, "{}", line)?;
    }

    Ok(())
}
