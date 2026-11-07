// The game tells entity ticks only to the mods that ask for them, and only
// while they are on.
fn zombies_are_running_listen(game: &mut crate::game::Game<'_>, listened: &mut bool) {
    use horizon_client_lib::zombies_are_running as m;
    if game.native_on(m::ID) {
        *listened = true;
    }
}
