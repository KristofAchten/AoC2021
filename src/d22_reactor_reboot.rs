use std::time::Instant;

pub fn reboot() -> String {
    let now = Instant::now();
    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", "Todo", "Todo", now.elapsed().as_millis());
}