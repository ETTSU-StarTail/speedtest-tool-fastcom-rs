use super::{model, utility};
use std::{error, fs, path};

pub fn check_exists_record_file(
    file_path: &path::Path,
    convert_byte: bool,
) -> Result<(), Box<dyn error::Error>> {
    let parent: Option<&path::Path> = file_path.parent();

    if parent.is_some() && !parent.unwrap().exists() {
        let parent_dir: &path::Path = parent.unwrap();
        match fs::create_dir_all(parent_dir) {
            Ok(()) => log::info!("Success create directory: {}", parent_dir.display()),
            Err(error) => {
                log::error!("Failed create directory.");
                log::error!("{:?}", error);
                panic!();
            }
        };
    }

    if !file_path.exists() {
        match fs::File::create(file_path) {
            Ok(_) => log::info!("Success create file: {}", file_path.display()),
            Err(error) => {
                log::error!("Failed create file.");
                log::error!("{:?}", error);
                panic!();
            }
        };

        let units: &str = if convert_byte { "Byte" } else { "Bit" };
        let file: fs::File = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();
        let mut w: csv::Writer<fs::File> = csv::Writer::from_writer(file);
        match w.write_record(&[
            "Tested Datetime",
            format!("Download Speed[M{}/s]", units).as_str(),
            format!("Upload Speed[M{}/s]", units).as_str(),
        ]) {
            Ok(_) => log::info!("Success record to file: {}", file_path.display()),
            Err(error) => {
                log::error!("Failed record to file.");
                log::error!("{:?}", error);
                panic!();
            }
        };
        w.flush()?;
    }

    Ok(())
}

pub fn write_line_to_csv(
    file_path: &path::Path,
    record: model::SpeedTestResultValues,
) -> Result<(), Box<dyn error::Error>> {
    let oo: fs::File = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();
    let mut w: csv::Writer<fs::File> = csv::Writer::from_writer(oo);

    w.write_record(&[
        record
            .tested_datetime
            .format("%Y-%m-%d %H:%m:%s")
            .to_string()
            .as_bytes(),
        record.download_speed_bps.to_string().as_bytes(),
        record.upload_speed_bps.to_string().as_bytes(),
    ])?;
    w.flush()?;

    Ok(())
}

pub fn format_tested_network_data(
    tested_network_data: model::SpeedTestResultValues,
) -> model::SpeedTestResultValues {
    let mut record: model::SpeedTestResultValues = tested_network_data.clone();

    record.tested_datetime = utility::round_datetime(record.tested_datetime);
    record.download_speed_bps =
        utility::change_order(record.download_speed_bps, utility::ValuePrefix::M);
    record.upload_speed_bps =
        utility::change_order(record.upload_speed_bps, utility::ValuePrefix::M);

    log::debug!("data for record to csv.");
    log::debug!("{:?}", record);

    return record;
}

pub fn record_to_csv(
    file_path: &path::Path,
    tested_network_data: model::SpeedTestResultValues,
    convert_byte: bool,
) -> Result<(), Box<dyn error::Error>> {
    log::info!("record to csv: {}", file_path.display());

    check_exists_record_file(file_path, convert_byte)?;

    let record: model::SpeedTestResultValues = format_tested_network_data(tested_network_data);
    write_line_to_csv(file_path, record)?;

    Ok(())
}
