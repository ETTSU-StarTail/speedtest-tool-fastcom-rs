pub fn init() {
    let log_path: &std::path::Path = std::path::Path::new("./log");
    if !log_path.exists() {
        std::fs::create_dir(log_path).unwrap();
    }

    let base_config = fern::Dispatch::new();

    let file_config = fern::Dispatch::new()
        .level(log::LevelFilter::Debug)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} - {} -> {}",
                record.level(),
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                message
            ))
        })
        .chain(
            fern::log_file(format!(
                "./log/{}.log",
                chrono::Local::now().format("%Y-%m-%d"),
            ))
            .unwrap(),
        );

    let stdout_config = fern::Dispatch::new()
        .level(log::LevelFilter::Warn)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {} - {} -> {}",
                record.level(),
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()
        .unwrap();
}
