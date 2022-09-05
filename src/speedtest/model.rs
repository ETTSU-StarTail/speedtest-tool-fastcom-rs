pub const SPEEDTEST_RESULT_HEADERS: [&str; 3] = [
    "Tested Datetime",
    "Download Speed[MBit/s]",
    "Upload Speed[MBit/s]",
];

/// speedtest result data
#[derive(Debug)]
pub struct SpeedTestResultValues {
    pub tested_datetime: chrono::DateTime<chrono::Local>,
    pub download_speed_mega_bps: f64,
    pub upload_speed_mega_bps: f64,
}

impl SpeedTestResultValues {
    pub fn new(
        tested_datetime: chrono::DateTime<chrono::Local>,
        download_speed_bps: f64,
        upload_speed_bps: f64,
    ) -> Self {
        Self {
            tested_datetime: tested_datetime,
            download_speed_mega_bps: download_speed_bps,
            upload_speed_mega_bps: upload_speed_bps,
        }
    }
}
