use std::time::Instant;

pub fn amphipod() -> String {
    let now = Instant::now();
    // Just played the game... https://amphipod.net/
    return format!("part 1 = `{}` ; part 2 = `{}` (time: {}ms)", 13520, 48708, now.elapsed().as_millis());
}