use std::error::Error;

use super::runner::Measurement;

const REPORTS_DIR: &str = "./gen/data";

pub fn remove_reports(name: &str) -> Result<(), Box<dyn Error>> {
    for entry in std::fs::read_dir(REPORTS_DIR).unwrap() {
        if entry
            .as_ref()
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .starts_with(name)
        {
            std::fs::remove_file(entry.unwrap().path())?;
        }
    }

    return Ok(());
}

pub fn write_report(name: &str, data: Vec<Measurement>) -> Result<(), Box<dyn Error>> {
    let file_name = format!("./gen/data/{}.csv", name);
    let mut wtr = csv::Writer::from_path(file_name)?;

    wtr.write_record(["n", "time"])?;
    for d in data {
        wtr.write_record([d.n.to_string(), d.time.as_nanos().to_string()])?;
    }

    wtr.flush()?;

    return Ok(());
}
