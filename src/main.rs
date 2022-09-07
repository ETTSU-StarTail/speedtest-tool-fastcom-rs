use speedtest_tool_fastcom_rs::{
    logger,
    speedtest::{controller, recorder, reporter},
};
use std::path;

/// network proxy settings.
#[derive(argh::FromArgs)]
struct Arguments {
    /// proxy server url.
    #[argh(option)]
    proxy_url: Option<String>,
    /// proxy bypass url.
    #[argh(option)]
    proxy_bypass: Option<String>,
    /// username for proxy authentication.
    #[argh(option)]
    proxy_username: Option<String>,
    /// password for proxy authentication.
    #[argh(option)]
    proxy_password: Option<String>,
    /// report upload path
    #[argh(option)]
    upload_path: String,
    /// report save path
    #[argh(option)]
    save_path: String,
    /// bit to byte
    #[argh(switch)]
    convert_byte: bool,
    /// force upload the report.
    #[argh(switch)]
    is_force: bool,
}

#[tokio::main]
async fn main() {
    logger::init();

    log::info!("speedtest tool start.");

    println!("Hello, world!");

    let arg: Arguments = argh::from_env();

    let result = controller::speedtest(
        arg.convert_byte,
        arg.proxy_url,
        arg.proxy_bypass,
        arg.proxy_username,
        arg.proxy_password,
    )
    .await
    .unwrap();

    let record_path: path::PathBuf = path::PathBuf::from(format!("{}/dest", arg.save_path));
    let upload_path: path::PathBuf = path::PathBuf::from(format!("{}/dest", arg.upload_path));
    let today: chrono::Date<chrono::Local> = chrono::Local::today();
    let file_path: path::PathBuf = path::PathBuf::new()
        .join(record_path.clone())
        .join(format!("{}_fastcom.csv", today.format("%Y-%m-%d")));
    recorder::record_to_csv(file_path.as_path(), result, arg.convert_byte).unwrap();

    let yesterday: chrono::Date<chrono::Local> = today - chrono::Duration::days(1);
    reporter::upload_report(
        record_path.as_path(),
        upload_path.as_path(),
        yesterday,
        arg.is_force,
    )
    .unwrap();

    log::info!("speedtest tool end.");
}
