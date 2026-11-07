// A timeline of six zombies over two minutes: `R` while a zombie runs at its
// prey, `.` while it catches its breath and walks. One character a second.
//
//     cargo run --example cycle
//
// The cycles are the mod's own code (this crate's library), so this is what the
// game drives: each zombie draws its own lengths from 12-20 s and 4-8 s.
use horizon_mod_zombies_are_running::{BREATH_TICKS, Bursts, SPRINT_TICKS, VERSION};

const SECONDS: usize = 120;
const TICKS_PER_SECOND: u32 = 20;

fn main() {
    let mut bursts = Bursts::default();
    let mut lines: Vec<String> = (0..6).map(|_| String::new()).collect();
    for _ in 0..SECONDS {
        for (row, line) in lines.iter_mut().enumerate() {
            let id = row as u32 + 1;
            let mut running = false;
            for _ in 0..TICKS_PER_SECOND {
                running = bursts.tick(id);
            }
            line.push(if running { 'R' } else { '.' });
        }
    }
    println!(
        "Zombies Are Running {VERSION} — {} to {} s of running, {} to {} s of walking",
        SPRINT_TICKS.0 / TICKS_PER_SECOND,
        SPRINT_TICKS.1 / TICKS_PER_SECOND,
        BREATH_TICKS.0 / TICKS_PER_SECOND,
        BREATH_TICKS.1 / TICKS_PER_SECOND,
    );
    for (row, line) in lines.iter().enumerate() {
        println!("zombie {} |{line}|", row + 1);
    }
    println!("         +{}+", "-".repeat(SECONDS));
    println!("         0         1         2         3         4         5         6         7         8         9       10       11");
    println!("         0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901");
}
