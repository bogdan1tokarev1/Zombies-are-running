// Every ticking zombie the server tells us about: advance its cycle.
fn zombies_are_running_tick(game: &mut crate::game::Game<'_>, entity: u32, kind: &str) {
    use horizon_client_lib::zombies_are_running as m;
    // Turned off in Options > Mods: clear the cycles, so no zombie is driven
    // until the next build leaves the mod out.
    if !game.native_on(m::ID) {
        m::reset();
        return;
    }
    let settings = {
        let game = &*game;
        m::settings(|key| game.native_setting(m::ID, key))
    };
    m::set_settings(settings);
    m::entity_ticked(entity, kind);
}
