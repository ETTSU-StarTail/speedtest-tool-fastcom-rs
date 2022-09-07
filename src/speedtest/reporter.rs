use std::{collections, error, fs, path};

/// Get past speedtest result for record to graph.
///
/// TODO: graph plotting
pub fn get_result_csv(
    file_path: &path::Path,
) -> Result<collections::HashMap<String, Vec<f64>>, Box<dyn error::Error>> {
    log::warn!("Not implement make_report.");

    Ok(collections::HashMap::new())
}

/// TODO: graph plotting.
pub fn make_network_speed_graph(
    csv_file_path: &path::Path,
    dest_path: &path::Path,
) -> Result<(), Box<dyn error::Error>> {
    log::warn!("Not implement make_report.");

    Ok(())
}

pub fn upload_report(
    save_dir_path: &path::Path,
    upload_dir_path: &path::Path,
    target_date: chrono::Date<chrono::Local>,
    is_force: bool,
) -> Result<(), Box<dyn error::Error>> {
    if !upload_dir_path.exists() {
        fs::create_dir_all(upload_dir_path)?;
    }

    let result_file_name: String = target_date.format("%Y-%m-%d").to_string() + "_fastcom";
    let save_result_file_path: path::PathBuf = path::PathBuf::new()
        .join(save_dir_path)
        .join(result_file_name.clone() + ".csv");
    let save_report_file_path: path::PathBuf = path::PathBuf::new()
        .join(save_dir_path)
        .join(result_file_name.clone() + ".html");

    if save_result_file_path.exists() {
        match fs::copy(
            save_result_file_path.clone(),
            upload_dir_path.join(result_file_name.clone() + ".csv"),
        ) {
            Ok(_) => log::info!("Success upload the result csv."),
            Err(error) => {
                log::error!("Failed upload the result csv.");
                log::error!("{:?}", error);
                panic!();
            }
        };
    } else {
        log::warn!(
            "result file is not found: {}",
            save_result_file_path.display()
        );
    }

    if save_report_file_path.exists() || is_force {
        // TODO: graph plotting.
        // make_network_speed_graph(
        //     save_result_file_path.as_path(),
        //     save_report_file_path.as_path(),
        // )?;

        // match fs::copy(save_report_file_path, upload_dir_path.join(result_file_name.clone() + ".html")) {
        //     Ok(_) => log::info!("Success upload the report html."),
        //     Err(error) => {
        //         log::error!("Failed upload the report html.");
        //         log::error!("{:?}", error);
        //         panic!();
        //     }
        // };
    } else {
        log::warn!("Not make report. Because exists report or is_force flag is false.")
    }

    Ok(())
}
