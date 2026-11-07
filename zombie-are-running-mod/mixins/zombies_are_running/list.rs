// The mod's row in Options > Mods.
fn zombies_are_running_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client_lib::zombies_are_running as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("{} {}", m::NAME, m::VERSION),
        description: format!("{} By {}.", m::DESCRIPTION, m::AUTHOR),
        keys: Vec::new(),
    });
}
