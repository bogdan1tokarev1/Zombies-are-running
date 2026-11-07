// A click on the mod's settings page: change the setting and keep it.
fn zombies_are_running_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use horizon_client_lib::zombies_are_running as m;
    if mod_id != m::ID {
        return;
    }
    let mut settings = m::settings(|key| game.native_setting(m::ID, key));
    settings.click(button);
    m::set_settings(settings);
    for (key, value) in settings.stored() {
        game.native_set_setting(m::ID, key, &value);
    }
}
