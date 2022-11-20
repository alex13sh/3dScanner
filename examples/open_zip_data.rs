use std::fs;
use zip::read::ZipFile;

fn main() {
    let file = fs::File::open("./data/2022-11-11_14-45-46.zip").unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    dbg!(archive.len());

    let file_accel = archive.by_name("Accelerometer.csv").unwrap();
    proc_accel(file_accel);

}


#[derive(Debug, serde::Deserialize)]
struct AccelRecord {
    Timestamp: String,
    Milliseconds: u32,
    X: f32,
    Y: f32,
    Z: f32,
}

fn proc_accel(file: ZipFile) {
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize() {
        let record: AccelRecord = result.unwrap();
        println!("{:?}", record);
    }
}
