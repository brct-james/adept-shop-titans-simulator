pub fn create_logfile() {
    // Create new log file each run
    let mut trial_logs_path: String;
    let mut i = 0;
    loop {
        trial_logs_path = f!("adept_data/logs/log_{}.log", i);
        // Increment until file not exist, then create and break
        if std::path::Path::new(&trial_logs_path).exists() {
            i += 1;
            continue;
        }
        break;
    }
    fast_log::init(fast_log::Config::new().file(&trial_logs_path)).unwrap();
    log::info!("Start of Log File");
}
