use speedtest_tool_fastcom_rs::{
    logger,
    speedtest::{controller, model},
};

#[tokio::main]
async fn main() {
    logger::init();

    log::debug!("speedtest tool start.");

    println!("Hello, world!");

    let result = controller::speedtest().await.unwrap();
    log::trace!(
        "{}: {}",
        model::SPEEDTEST_RESULT_HEADERS[0],
        result.tested_datetime
    );
    log::trace!(
        "{}: {}",
        model::SPEEDTEST_RESULT_HEADERS[1],
        result.download_speed_mega_bps
    );
    log::trace!(
        "{}: {}",
        model::SPEEDTEST_RESULT_HEADERS[2],
        result.upload_speed_mega_bps
    );

    log::debug!("speedtest tool end.");
}
