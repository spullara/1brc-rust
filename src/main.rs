use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

#[derive(Debug)]
struct Result {
    min: f64,
    max: f64,
    sum: f64,
    count: u64,
}

fn main() -> io::Result<()> {
    let start = Instant::now();
    let file = File::open("./measurements.txt")?;
    let reader = BufReader::new(file);

    let results = reader.lines().filter_map(|line| line.ok()).map(|l| {
        let parts: Vec<&str> = l.split(';').collect();
        (parts[0].to_string(), parts[1].parse::<f64>().unwrap())
    }).fold(BTreeMap::new(), |mut acc: BTreeMap<String, Result>, (key, temperature)| {
        let entry = acc.entry(key).or_insert_with(|| Result {
            min: temperature,
            max: temperature,
            sum: 0.0,
            count: 0,
        });
        entry.min = entry.min.min(temperature);
        entry.max = entry.max.max(temperature);
        entry.sum += temperature;
        entry.count += 1;
        acc
    });

    println!("{:?}", Instant::now().duration_since(start));
    println!("{:?}", results);
    Ok(())
}
