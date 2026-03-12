use std::sync::LazyLock;
use std::time::Duration;

fn is_no_sleep() -> bool {
    let mut args = std::env::args();
    let Some(arg) = args.nth(1) else { return false };
    &arg == "nosleep"
}

static NO_SLEEP: LazyLock<bool> = LazyLock::new(is_no_sleep);

pub fn sleep_ms(ms: u64) {
    if *NO_SLEEP {
        return;
    }

    std::thread::sleep(Duration::from_millis(ms));
}
