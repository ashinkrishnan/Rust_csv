use csv::Error;
use csv::Writer;
use std::io;

fn main() -> Result<(), Error> {
    let mut reader = csv::Reader::from_reader(io::stdin());
    let mut count = 0;
    let mut wtr_odd = Writer::from_path("odd_rows.csv")?;
    wtr_odd.write_record(reader.headers()?)?;

    let mut wtr_even = Writer::from_path("even_rows.csv")?;
    wtr_even.write_record(reader.headers()?)?;

    for record in reader.records() {
        let mut recvec: Vec<String> = Vec::new();
        let record = record?;
        if count % 2 == 0 {
            println!("{:?}, even row count {}", record, count);
            for i in 0..record.len() {
                recvec.push(record.get(i).unwrap().to_string());
            }
            wtr_even.write_record(&recvec)?;
            wtr_even.flush()?;
        }
        if count % 2 != 0 {
            println!("{:?}, odd row count {}, ", record, count);
            let recoddvec: Vec<String> = (0..record.len())
                .map(|x| record.get(x).unwrap().to_string())
                .collect();
            wtr_odd.write_record(&recoddvec)?;
            wtr_odd.flush()?;
        }
        count += 1;
    }
    println!("{}", count);

    Ok(())
}
