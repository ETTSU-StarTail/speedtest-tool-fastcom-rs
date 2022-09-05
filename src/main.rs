mod logger;

fn main() {
    logger::init();

    log::debug!("speedtest tool start.");

    println!("Hello, world!");

    log::debug!("speedtest tool end.");
}
