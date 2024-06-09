use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

async fn line_count(filename: String) -> anyhow::Result<usize> {
    let now = std::time::Instant::now();
    let mut line_count: i32 = 0;
    if let Ok(lines) = read_lines(filename) {
        lines.for_each(|line| {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    line_count += 1;
                }
            }
        });
    }
    println!(
        "Read {} in lines {:.3} in seconds",
        line_count,
        now.elapsed().as_secs_f32()
    );
    // Ok(line_count)
    Ok(line_count.try_into().unwrap())
}

async fn async_line_count(filename: String) -> anyhow::Result<usize> {
    use tokio::fs::File;
    use tokio::io::AsyncBufReadExt;
    use tokio::io::BufReader;

    println!("Reading {filename} ...");
    let now = Instant::now();
    let mut line_count = 0;

    let file = File::open(filename).await?;

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if !line.trim().is_empty() {
            line_count += 1;
        }
    }
    println!(
        "Read {} in lines {:.3} in seconds",
        line_count,
        now.elapsed().as_secs_f32()
    );
    // Ok(line_count)
    Ok(line_count.try_into().unwrap())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Reading readme ...");
    let now = std::time::Instant::now();
    let (c1, c2, ..) = tokio::join!(
        async_line_count("../Cargo.toml".to_string()),
        async_line_count("../Cargo.toml".to_string()),
        async_line_count("../Cargo.toml".to_string()),
        async_line_count("../Cargo.toml".to_string()),
    );
    println!("Totoal lines: {}", c1? + c2?);
    println!("In {:.3} seconds", now.elapsed().as_secs_f32());
    Ok(())
}
