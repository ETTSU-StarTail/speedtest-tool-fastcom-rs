use speedtest_tool_fastcom_rs::{
    logger,
    speedtest::{controller, model},
};

/// network proxy settings.
#[derive(argh::FromArgs)]
struct ProxySettings {
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
}

#[tokio::main]
async fn main() {
    logger::init();

    log::info!("speedtest tool start.");

    println!("Hello, world!");

    let arg: ProxySettings = argh::from_env();

    let result = controller::speedtest(
        arg.proxy_url,
        arg.proxy_bypass,
        arg.proxy_username,
        arg.proxy_password,
    )
    .await
    .unwrap();
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

    log::info!("speedtest tool end.");
}
