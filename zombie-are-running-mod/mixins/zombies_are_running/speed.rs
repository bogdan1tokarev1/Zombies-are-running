// A zombie's movement speed, as its burst has it: faster while it runs at its
// prey, slower while it catches its breath. Everyone else keeps its own.
fn zombies_are_running_speed(value: f64, mob: &crate::mobs::Mob) -> f64 {
    use crate::zombies_are_running as m;
    // A zombie that has nothing to chase moves as the game made it.
    if mob.ai.target.is_none() {
        return value;
    }
    match m::chase_factor(mob.id) {
        Some(factor) => value * f64::from(factor),
        None => value,
    }
}
