use std::error::Error;

// Write to a file
pub fn write_to_file<T>(data_line: &T) -> Result<(), Box<dyn Error>>
where
    T: IntoIterator<Item = dyn std::string::ToString> + ExactSizeIterator,
{
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("test_data/data.tsv")
        .unwrap();

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_writer(file);

    let mut data_line_str: Vec<String> = Vec::with_capacity(data_line.len());

    data_line
        .iter()
        .for_each(|x| data_line_str.push(x.to_string()));

    wtr.write_record(&data_line_str)?;

    wtr.flush()?;
    Ok(())
}

// User assertions to check for problems
#[test]
fn test_write() -> Result<(), Box<dyn Error>>
{
    let test: Vec<f64> = vec![1.423, 0.61324, 123.865];
    write_to_file(&test)?;
    // Could use assert_eq! and open file and check matching
    let _remove_success = std::fs::remove_file("test_data/data.tsv");
    Ok(())
}

// Read from a file
use std::io::Read;
pub fn read_from_file()
{
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
