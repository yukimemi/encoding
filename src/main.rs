use csv::{self, StringRecord};
use encoding_rs_io::DecodeReaderBytes;
use failure::Error;
use std::fs;
use std::io;
use std::path::Path;
type Result<T> = std::result::Result<T, Error>;

fn main() {
    println!("Hello, world!");
}

fn read_csv<P: AsRef<Path>>(path: P) -> Result<csv::Reader<fs::File>> {
    let f = fs::File::open(path)?;
    let rdr = csv::Reader::from_reader(f);
    Ok(rdr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_read_csv() -> Result<()> {
        let mut csv_path = env::current_dir().unwrap();
        csv_path.push("test");
        csv_path.push("test.csv");
        if let Ok(mut rdr) = read_csv(csv_path) {
            rdr.records()
                .map(|r| -> Result<StringRecord> {
                    let r = r?;
                    dbg!(&r);
                    Ok(r)
                })
                .collect::<Vec<_>>();
        }
        Ok(())
    }
}
