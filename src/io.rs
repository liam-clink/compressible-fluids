use std::error::Error;

// Write to a file
// This accepts types that can be turned into iterators that know their size,
// and whose corresponding type can be converted to a string
pub fn write_to_file<T, I>(data_line: T) -> Result<(), Box<dyn Error>>
where
    T: IntoIterator<IntoIter = I>,
    // The ExactSizeIterator could be just Iterator due to the use of collect() instead of for_each(),
    // but if I add checks based on size, this will be required
    I: ExactSizeIterator,
    <I as Iterator>::Item: ToString,
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

    // into_iter() is used instead of iter() because iter() doesn't have a trait
    let data_line_str: Vec<String> = data_line.into_iter().map(|x| x.to_string()).collect();

    wtr.write_record(&data_line_str)?;

    wtr.flush()?;
    Ok(())
}

// Use assertions to check for problems
#[test]
fn test_write() -> Result<(), Box<dyn Error>>
{
    let test_vec: Vec<f64> = vec![1.423, 0.61324, 123.865];
    let _remove_success = std::fs::remove_file("test_data/data.tsv");
    write_to_file(test_vec)?;
    // Could use assert_eq! and open file and check matching
    let _remove_success = std::fs::remove_file("test_data/data.tsv");
    Ok(())
}

// Read from a file
use std::io::Read;
pub fn read_from_file<A, B>(fpath: &str) -> Result<A, B::Err>
where
    A: FromIterator<B>,
    B: std::str::FromStr,
{
    let mut file = std::fs::File::open(fpath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let result = contents
        .split('\t')
        .map(|x| {
            let element = x.parse::<B>();
            match element
            {
                Ok(valid_element) =>
                {
                    return valid_element;
                }
                Err(err) =>
                {
                    eprintln!("Element in input file is not valid: {err}");
                }
            }
        })
        .collect();
    result
}

#[test]
fn test_read() -> Result<(), Box<dyn Error>>
{
    let test_vec: Vec<f64> = read_from_file("test_data/data.tsv");

    Ok(())
}
