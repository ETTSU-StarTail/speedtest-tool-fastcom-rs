use super::{model, utility};
use std::{error, fs, path};

pub fn check_exists_record_file(
    file_path: &path::Path,
    convert_byte: bool,
) -> Result<(), Box<dyn error::Error>> {
    let parent: Option<&path::Path> = file_path.parent();

    if parent.is_some() {
        let parent_dir: &path::Path = parent.unwrap();
        fs::create_dir_all(parent_dir)?;
    }

    if !file_path.exists() {
        let units: &str = if convert_byte { "Byte" } else { "Bit" };
        fs::File::create(file_path)?;

        let mut w: csv::Writer<fs::File> = csv::Writer::from_path(file_path).unwrap();
        w.write_record(&[
            "Tested Datetime",
            format!("Download Speed[M{}/s]", units).as_str(),
            format!("Upload Speed[M{}/s]", units).as_str(),
        ])?;
        w.flush()?;
    }

    Ok(())
}

pub fn write_line_to_csv(
    file_path: &path::Path,
    record: model::SpeedTestResultValues,
) -> Result<(), Box<dyn error::Error>> {
    let parent: Option<&path::Path> = file_path.parent();
    if parent.is_some() && !file_path.exists() {
        let mut w: csv::Writer<fs::File> = csv::Writer::from_path(file_path).unwrap();

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
    }

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

    return record;
}

pub fn record_to_csv(
    file_path: &path::Path,
    tested_network_data: model::SpeedTestResultValues,
    convert_byte: bool,
) -> Result<(), Box<dyn error::Error>> {
    check_exists_record_file(file_path, convert_byte)?;

    let record: model::SpeedTestResultValues = format_tested_network_data(tested_network_data);
    write_line_to_csv(file_path, record)?;

    Ok(())
}
