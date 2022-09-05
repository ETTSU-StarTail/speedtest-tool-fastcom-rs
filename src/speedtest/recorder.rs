use super::model;
use std::path;

pub fn check_exists_record_file() -> bool {
    true
}

pub fn write_line_to_csv(
    dir_path: &path::Path,
    record: model::SpeedTestResultValues,
    convert_byte: bool,
) {
}

pub fn format_tested_network_data(
    tested_network_data: model::SpeedTestResultValues,
) -> [String; 3] {
    ["", "", ""].map(|v| String::from(v))
}

pub fn record_to_csv(
    file_path: &path::Path,
    tested_network_data: model::SpeedTestResultValues,
    convert_byte: bool,
) {
}
