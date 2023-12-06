// Initial File stolen from Marie (https://github.com/NyCodeGHG/advent-of-code-2022/blob/main/src/bin/create-day.rs) 😊

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

macro_rules! create_and_write {
    ($path:literal, $text:literal) => {
        create_and_write_impl(format!($path), format!($text))
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read command line argument for the day and parse it into a number leading zeros
    let day = format!(
        "{:02}",
        std::env::args()
            .nth(1)
            .ok_or("Missing arguments")?
            .parse::<u8>()?
    );

    // Create the file if it doesn't exist
    create_and_write!("input/day{day}.txt", "")?;
    create_and_write!(
        "src/bin/day{day}.rs",
        "pub fn main() {{
    let input = include_str!(\"../../input/day{day}.txt\");
}}
"
    )?;
    create_and_write!(
        "benches/day{day}/main.rs",
        "use criterion::{{criterion_group, criterion_main, Criterion}};

#[path = \"../../src/bin/day{day}.rs\"]
mod day{day};

pub fn criterion_benchmark(c: &mut Criterion) {{
    c.bench_function(\"day {day}\", |b| b.iter(day{day}::main));
}}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

"
    )?;

    let mut cargo_toml = File::open("Cargo.toml")?;
    cargo_toml.write_all(
        format!(
            "\
[[bench]]
name = \"day{day}\"
harness = false
"
        )
        .as_bytes(),
    )?;

    Ok(())
}

fn create_and_write_impl<P: AsRef<Path>>(path: P, content: String) -> Result<(), std::io::Error> {
    if path.as_ref().exists() {
        eprintln!("File already exists: {}", path.as_ref().display());
        return Ok(());
    }

    fs::create_dir_all(path.as_ref().parent().unwrap())?;
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    eprintln!("Created file: {}", path.as_ref().display());
    Ok(())
}
