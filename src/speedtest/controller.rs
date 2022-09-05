use std::path::PathBuf;

use super::model;

pub async fn speedtest() -> Result<model::SpeedTestResultValues, playwright::Error> {
    let result: model::SpeedTestResultValues = model::SpeedTestResultValues {
        tested_datetime: chrono::Local::now(),
        download_speed_mega_bps: 0.0,
        upload_speed_mega_bps: 0.0,
    };

    let playwright = playwright::Playwright::initialize().await?;
    playwright.prepare()?;
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    page.goto_builder("https://google.com/").goto().await?;

    page.screenshot_builder()
        .path(PathBuf::from("./log/screenshot.png"))
        .screenshot()
        .await?;

    Ok(result)
}
