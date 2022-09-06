use std::net;

/// speedtest result data
#[derive(Debug, Clone)]
pub struct SpeedTestResultValues {
    pub tested_datetime: chrono::DateTime<chrono::Local>,
    pub download_speed_bps: f64,
    pub upload_speed_bps: f64,
}

impl SpeedTestResultValues {
    pub fn new(
        tested_datetime: chrono::DateTime<chrono::Local>,
        download_speed_bps: f64,
        upload_speed_bps: f64,
    ) -> Self {
        Self {
            tested_datetime,
            download_speed_bps,
            upload_speed_bps,
        }
    }
}

/// fast.com data
#[derive(Debug)]
pub struct FastComData {
    pub is_done: bool,
    pub download_speed: f64,
    pub download_units: String,
    pub downloaded: f64,
    pub upload_speed: f64,
    pub upload_units: String,
    pub uploaded: f64,
    pub latency: f64,
    pub buffer_bloat: f64,
    pub user_location: String,
    pub user_ip: net::Ipv4Addr,
}
impl FastComData {
    pub fn new() -> Self {
        Self {
            is_done: false,
            download_speed: 0.0,
            download_units: String::from(""),
            downloaded: 0.0,
            upload_speed: 0.0,
            upload_units: String::from(""),
            uploaded: 0.0,
            latency: 0.0,
            buffer_bloat: 0.0,
            user_location: String::from(""),
            user_ip: net::Ipv4Addr::new(0, 0, 0, 0),
        }
    }
}
