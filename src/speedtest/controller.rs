use super::model;
use playwright::api;
use std::path::PathBuf;

pub async fn setting_browser(
    browser_type: &api::BrowserType,
    proxy_url: Option<String>,
    proxy_bypass: Option<String>,
    proxy_username: Option<String>,
    proxy_password: Option<String>,
) -> Result<api::Browser, playwright::Error> {
    let browser: api::Browser;

    if proxy_url.is_none() {
        log::info!("speedtest without proxy.");
        browser = browser_type.launcher().headless(true).launch().await?;
    } else {
        log::info!("speedtest with proxy.");
        let proxy_settings: api::ProxySettings = api::ProxySettings {
            server: proxy_url.unwrap(),
            bypass: proxy_bypass,
            username: proxy_username,
            password: proxy_password,
        };

        browser = browser_type
            .launcher()
            .headless(true)
            .proxy(proxy_settings)
            .launch()
            .await?;
    }

    Ok(browser)
}

pub async fn speedtest(
    proxy_url: Option<String>,
    proxy_bypass: Option<String>,
    proxy_username: Option<String>,
    proxy_password: Option<String>,
) -> Result<model::SpeedTestResultValues, playwright::Error> {
    log::info!("testing network speed.");

    log::warn!("------------- [CAUTION] -----------");
    log::warn!("Trial implement.");
    log::warn!("Launch browser by playwright-rust.");
    log::warn!("And capture full page screenshot.");
    log::warn!("------------- [CAUTION] -----------");

    let result: model::SpeedTestResultValues = model::SpeedTestResultValues {
        tested_datetime: chrono::Local::now(),
        download_speed_mega_bps: 0.0,
        upload_speed_mega_bps: 0.0,
    };

    let playwright: playwright::Playwright = playwright::Playwright::initialize().await?;
    playwright.prepare()?;
    let chromium: api::BrowserType = playwright.chromium();
    let browser: api::Browser = setting_browser(
        &chromium,
        proxy_url,
        proxy_bypass,
        proxy_username,
        proxy_password,
    )
    .await?;
    let context: api::BrowserContext = browser.context_builder().build().await?;
    let page: api::Page = context.new_page().await?;
    page.goto_builder("https://google.com/").goto().await?;

    page.screenshot_builder()
        .path(PathBuf::from("./log/screenshot.png"))
        .screenshot()
        .await?;

    log::info!("tested network speed.");

    Ok(result)
}
