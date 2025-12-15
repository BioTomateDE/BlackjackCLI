use std::time::Duration;

pub fn sleep_ms(ms: u64) {
    if std::env::args().nth(1).as_deref() == Some("nosleep") {
        return;
    }
    let sleep_mode = std::env::var("BJ_SLEEP").unwrap_or_default();
    match sleep_mode.as_str() {
        "disabled" | "0" | "false" => return,
        _ => {}
    }
    std::thread::sleep(Duration::from_millis(ms));
}
