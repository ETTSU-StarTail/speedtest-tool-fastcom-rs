use super::{model, utility};
use playwright::api;
use std::str::FromStr;
use std::{net, thread, time};

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
        browser = browser_type
            .launcher()
            .channel(api::BrowserChannel::Msedge)
            .headless(true)
            .launch()
            .await?;
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
            .channel(api::BrowserChannel::Msedge)
            .headless(true)
            .proxy(proxy_settings)
            .launch()
            .await?;
    }

    Ok(browser)
}

pub async fn get_network_speed_bu_fastcom(
    proxy_url: Option<String>,
    proxy_bypass: Option<String>,
    proxy_username: Option<String>,
    proxy_password: Option<String>,
) -> Result<model::FastComData, playwright::Error> {
    log::info!("testing network speed.");

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
    page.goto_builder("https://fast.com/").goto().await?;

    let mut fastcom_data: model::FastComData = model::FastComData::new();

    while !fastcom_data.is_done {
        fastcom_data.download_speed = f64::from_str(
            page.text_content("#speed-value", None)
                .await?
                .unwrap()
                .trim(),
        )
        .unwrap();
        fastcom_data.download_units = page
            .text_content("#speed-units", None)
            .await?
            .unwrap()
            .trim()
            .to_string();
        fastcom_data.downloaded = f64::from_str(
            page.text_content("#down-mb-value", None)
                .await?
                .unwrap()
                .trim(),
        )
        .unwrap();
        fastcom_data.upload_speed = f64::from_str(
            page.text_content("#upload-value", None)
                .await?
                .unwrap()
                .trim(),
        )
        .unwrap();
        fastcom_data.upload_units = page
            .text_content("#upload-units", None)
            .await?
            .unwrap()
            .trim()
            .to_string();
        fastcom_data.uploaded = f64::from_str(
            page.text_content("#up-mb-value", None)
                .await?
                .unwrap()
                .trim(),
        )
        .unwrap();
        fastcom_data.latency = f64::from_str(
            page.text_content("#latency-value", None)
                .await?
                .unwrap()
                .trim(),
        )
        .unwrap();
        fastcom_data.buffer_bloat = f64::from_str(
            page.text_content("#bufferbloat-value", None)
                .await?
                .unwrap()
                .trim(),
        )
        .unwrap();
        fastcom_data.user_location = page
            .text_content("#user-location", None)
            .await?
            .unwrap()
            .trim()
            .to_string();
        fastcom_data.user_ip = page.text_content("#user-ip", None).await?.unwrap();
        fastcom_data.is_done = page
            .query_selector("#speed-value.succeeded")
            .await?
            .is_some()
            && page
                .query_selector("#upload-value.succeeded")
                .await?
                .is_some();

        log::debug!("{:?}", fastcom_data);

        thread::sleep(time::Duration::from_secs(5));
    }

    log::trace!("got network speed.");

    Ok(fastcom_data)
}

pub async fn speedtest(
    convert_byte: bool,
    proxy_url: Option<String>,
    proxy_bypass: Option<String>,
    proxy_username: Option<String>,
    proxy_password: Option<String>,
) -> Result<model::SpeedTestResultValues, playwright::Error> {
    let test_datetime: chrono::DateTime<chrono::Local> = chrono::Local::now();

    let tested_data: model::FastComData =
        get_network_speed_bu_fastcom(proxy_url, proxy_bypass, proxy_username, proxy_password)
            .await?;

    let mut result: model::SpeedTestResultValues = model::SpeedTestResultValues {
        tested_datetime: test_datetime,
        download_speed_bps: utility::clear_order(
            tested_data.download_speed,
            tested_data
                .download_units
                .chars()
                .next()
                .unwrap()
                .to_string()
                .as_str(),
        ),
        upload_speed_bps: utility::clear_order(
            tested_data.upload_speed,
            tested_data
                .upload_units
                .chars()
                .next()
                .unwrap()
                .to_string()
                .as_str(),
        ),
    };

    log::debug!("tested data(cleared order).");
    log::debug!("{:?}", result);

    if convert_byte {
        result.download_speed_bps = utility::bits_to_byte(result.download_speed_bps);
        result.upload_speed_bps = utility::bits_to_byte(result.upload_speed_bps);
    }

    Ok(result)
}
