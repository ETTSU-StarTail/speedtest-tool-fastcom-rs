use std::{collections, path};

pub fn get_result_csv(file_path: &path::Path) -> collections::HashMap<String, Vec<f64>> {
    collections::HashMap::new()
}

pub fn make_network_speed_graph(csv_file_path: &path::Path, dest_path: &path::Path) {}

pub fn upload_report(
    record_dir_path: &path::Path,
    upload_dir_path: &path::Path,
    target_date: chrono::Date<chrono::Local>,
    is_force: Option<bool>,
) {
}
