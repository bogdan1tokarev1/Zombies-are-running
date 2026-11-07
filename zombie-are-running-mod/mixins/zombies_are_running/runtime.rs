// Zombies Are Running 1.1.0, a Horizon native mod by Bogdan_Tokarev: a zombie that
// chases a player does it in bursts. It runs at the player for 12 to 20
// seconds, then catches its breath for 4 to 8 seconds, walking after the
// player instead of running, and then runs again. Each zombie draws its own
// lengths, so a group of them never sprints in step.
//
// This file is both the mod's Cargo library (its tests, `cargo test`) and the
// module the bundle adds to the game as `crate::zombies_are_running`.

/// The mod's id (its row, its settings folder), name, version and author.
pub const ID: &str = "zombies_are_running";
pub const NAME: &str = "Zombies Are Running";
pub const VERSION: &str = "1.1.0";
pub const AUTHOR: &str = "Bogdan_Tokarev";
pub const DESCRIPTION: &str =
    "Zombies chase you in bursts: 12 to 20 seconds of running, then 4 to 8 seconds of walking to catch their breath.";

/// The kinds the mod can drive, plain zombies first.
pub const FAMILY: [&str; 4] = [
    "minecraft:zombie",
    "minecraft:husk",
    "minecraft:drowned",
    "minecraft:zombie_villager",
];

/// A run lasts 12 to 20 seconds and a breath 4 to 8, in ticks (20 a second).
pub const SPRINT_TICKS: (u32, u32) = (12 * 20, 20 * 20);
pub const BREATH_TICKS: (u32, u32) = (4 * 20, 8 * 20);

/// How much faster a running zombie chases, and how much slower one that walks.
pub const SPRINT_FACTORS: [f32; 5] = [1.5, 2.0, 2.5, 3.0, 3.5];
pub const BREATH_FACTORS: [f32; 4] = [0.4, 0.5, 0.6, 0.75];
const DEFAULT_SPRINT: usize = 1;
const DEFAULT_BREATH: usize = 1;

/// A mob that stopped ticking is forgotten after a minute.
pub const FORGET_TICKS: u32 = 1200;

pub const SETTING_SPRINT: &str = "sprint";
pub const SETTING_BREATH: &str = "breath";
pub const SETTING_FAMILY: &str = "family";

/// The mod's settings, as its settings page keeps them (one file per key).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    /// Index in `SPRINT_FACTORS`.
    pub sprint: usize,
    /// Index in `BREATH_FACTORS`.
    pub breath: usize,
    /// Drive the whole `FAMILY`, not `minecraft:zombie` alone. On by default:
    /// a zombie, a husk, a drowned and a zombie villager all run in bursts.
    pub family: bool,
}

impl Settings {
    /// From the stored values; the defaults for the ones missing.
    pub fn read(get: impl Fn(&str) -> Option<String>) -> Self {
        let index = |key: &str, values: usize, default: usize| {
            get(key)
                .and_then(|value| value.trim().parse::<usize>().ok())
                .filter(|index| *index < values)
                .unwrap_or(default)
        };
        Self {
            sprint: index(SETTING_SPRINT, SPRINT_FACTORS.len(), DEFAULT_SPRINT),
            breath: index(SETTING_BREATH, BREATH_FACTORS.len(), DEFAULT_BREATH),
            // The whole family unless the player asks for plain zombies alone.
            family: get(SETTING_FAMILY).is_none_or(|value| value.trim() != "zombie"),
        }
    }
    pub fn sprint_factor(&self) -> f32 {
        SPRINT_FACTORS[self.sprint]
    }
    pub fn breath_factor(&self) -> f32 {
        BREATH_FACTORS[self.breath]
    }
    /// The movement factor of a mob that runs (`sprinting`) or walks.
    pub fn factor(&self, sprinting: bool) -> f32 {
        if sprinting {
            self.sprint_factor()
        } else {
            self.breath_factor()
        }
    }
    /// Whether the mod drives an entity of this type.
    pub fn drives(&self, kind: &str) -> bool {
        if self.family {
            FAMILY.contains(&kind)
        } else {
            kind == FAMILY[0]
        }
    }
    /// The labels of the settings page's buttons.
    pub fn labels(&self) -> Vec<String> {
        vec![
            format!("Run speed: x{:.1} (12 to 20 s)", self.sprint_factor()),
            format!("Walking speed: x{:.1} (4 to 8 s)", self.breath_factor()),
            format!(
                "Zombies: {}",
                if self.family {
                    "zombie, husk, drowned, zombie villager"
                } else {
                    "zombie"
                }
            ),
        ]
    }
    pub fn click(&mut self, button: usize) {
        match button {
            0 => self.sprint = (self.sprint + 1) % SPRINT_FACTORS.len(),
            1 => self.breath = (self.breath + 1) % BREATH_FACTORS.len(),
            2 => self.family = !self.family,
            _ => {}
        }
    }
    /// What the settings page writes back, one file per key.
    pub fn stored(&self) -> [(&'static str, String); 3] {
        [
            (SETTING_SPRINT, self.sprint.to_string()),
            (SETTING_BREATH, self.breath.to_string()),
            (
                SETTING_FAMILY,
                if self.family { "family" } else { "zombie" }.to_owned(),
            ),
        ]
    }
}

/// One mob's cycle of runs and breaths.
#[derive(Clone, Debug)]
pub struct Burst {
    /// Whether it runs at its prey now.
    pub sprinting: bool,
    /// Ticks left in this phase.
    left: u32,
    rng: u64,
}

impl Burst {
    /// A cycle that starts with a breath, as a zombie that just spawned does.
    pub fn new(seed: u64) -> Self {
        let mut burst = Self {
            sprinting: false,
            left: 0,
            rng: seed | 1,
        };
        burst.left = burst.duration();
        burst
    }
    /// xorshift64*: the cycle's own random numbers, so the mod needs no
    /// dependency and its tests are repeatable.
    fn next(&mut self) -> u32 {
        let mut x = self.rng;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.rng = x;
        (x.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }
    /// The length of the phase it is in: 12 to 20 s of running, 4 to 8 s of walking.
    fn duration(&mut self) -> u32 {
        let (low, high) = if self.sprinting {
            SPRINT_TICKS
        } else {
            BREATH_TICKS
        };
        low + self.next() % (high - low + 1)
    }
    /// One tick: what it does now. The phase it is in lasts exactly the ticks
    /// it drew, so 12 to 20 seconds of running and 4 to 8 of walking.
    pub fn tick(&mut self) -> bool {
        if self.left == 0 {
            self.sprinting = !self.sprinting;
            self.left = self.duration();
        }
        self.left -= 1;
        self.sprinting
    }
}

/// Every driven mob of one game (the window's, or the single-player server's).
#[derive(Debug, Default)]
pub struct Bursts {
    now: u32,
    mobs: std::collections::HashMap<u32, (Burst, u32)>,
}

impl Bursts {
    /// Ticks counted so far, for `sweep`.
    pub fn now(&self) -> u32 {
        self.now
    }
    /// One tick of the mob `id`: what it does now.
    pub fn tick(&mut self, id: u32) -> bool {
        self.now = self.now.wrapping_add(1);
        let now = self.now;
        let (burst, seen) = self
            .mobs
            .entry(id)
            .or_insert_with(|| (Burst::new(seed(id)), now));
        *seen = now;
        burst.tick()
    }
    /// How much faster the mob `id` moves now, for a mob the mod drives.
    pub fn factor(&self, id: u32, settings: &Settings) -> Option<f32> {
        self.mobs
            .get(&id)
            .map(|(burst, _)| settings.factor(burst.sprinting))
    }
    /// Stops driving the mob `id`: it died, left, or its kind isn't driven.
    pub fn forget(&mut self, id: u32) {
        self.mobs.remove(&id);
    }
    /// Stops driving every mob.
    pub fn clear(&mut self) {
        self.mobs.clear();
    }
    /// Forgets the mobs that stopped ticking.
    pub fn sweep(&mut self) {
        let now = self.now;
        self.mobs
            .retain(|_, (_, seen)| now.wrapping_sub(*seen) < FORGET_TICKS);
    }
}

/// A cycle's first seed: the mob's id mixed, so that neighbours differ.
fn seed(id: u32) -> u64 {
    let mixed = u64::from(id).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    (mixed ^ 0x5DEE_CE66_D1CE_1234) | 1
}

thread_local! {
    /// The settings, read once from the files, then kept.
    static SETTINGS: std::cell::Cell<Option<Settings>> = const { std::cell::Cell::new(None) };
    /// The driven mobs of this game.
    static BURSTS: std::cell::RefCell<Bursts> = std::cell::RefCell::new(Bursts::default());
}

/// The settings, read once through `get` (one file per key).
pub fn settings(get: impl Fn(&str) -> Option<String>) -> Settings {
    SETTINGS.with(|cached| {
        let found = cached.get().unwrap_or_else(|| Settings::read(get));
        cached.set(Some(found));
        found
    })
}

/// Replaces the settings (the settings page changed them).
pub fn set_settings(new: Settings) {
    SETTINGS.with(|cached| cached.set(Some(new)));
}

/// The mob `id`, of type `kind`, ticked: advance its cycle.
pub fn entity_ticked(id: u32, kind: &str) {
    let Some(settings) = SETTINGS.with(|cached| cached.get()) else {
        return;
    };
    BURSTS.with(|bursts| {
        let mut bursts = bursts.borrow_mut();
        if settings.drives(kind) {
            bursts.tick(id);
        } else {
            bursts.forget(id);
        }
        if bursts.now() % FORGET_TICKS == 0 {
            bursts.sweep();
        }
    });
}

/// How much faster the mob `id` moves now: none, for a mob the mod doesn't drive.
pub fn chase_factor(id: u32) -> Option<f32> {
    let settings = SETTINGS.with(|cached| cached.get())?;
    BURSTS.with(|bursts| bursts.borrow().factor(id, &settings))
}

/// The mod is off: no mob is driven any more.
pub fn reset() {
    BURSTS.with(|bursts| bursts.borrow_mut().clear());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stored<'a>(pairs: &'a [(&'a str, &'a str)]) -> impl Fn(&str) -> Option<String> + 'a {
        |key| {
            pairs
                .iter()
                .find(|pair| pair.0 == key)
                .map(|pair| pair.1.to_owned())
        }
    }

    /// The ticks of each phase, in order, over `ticks` ticks.
    fn phases(burst: &mut Burst, ticks: usize) -> Vec<(bool, u32)> {
        let mut out: Vec<(bool, u32)> = Vec::new();
        for _ in 0..ticks {
            let sprinting = burst.tick();
            match out.last_mut() {
                Some((phase, length)) if *phase == sprinting => *length += 1,
                _ => out.push((sprinting, 1)),
            }
        }
        out
    }

    #[test]
    fn runs_and_breaths_stay_in_the_documented_ranges() {
        for id in 0..64u32 {
            let mut burst = Burst::new(seed(id));
            let mut seen = phases(&mut burst, 20 * 60 * 4);
            // The window ends in the middle of the last phase.
            seen.pop();
            for (sprinting, length) in seen {
                let (low, high) = if sprinting { SPRINT_TICKS } else { BREATH_TICKS };
                assert!(
                    (low..=high).contains(&length),
                    "a {} phase of {length} ticks",
                    if sprinting { "running" } else { "walking" }
                );
            }
        }
    }

    #[test]
    fn a_zombie_starts_by_catching_its_breath() {
        let mut burst = Burst::new(seed(7));
        assert!(!burst.sprinting, "it starts walking");
        assert!(BREATH_TICKS.0 <= burst.left && burst.left <= BREATH_TICKS.1);
        let (sprinting, length) = phases(&mut burst, 1)[0];
        assert!(!sprinting && length == 1);
    }

    #[test]
    fn neighbours_do_not_share_a_cycle() {
        let mut first = Burst::new(seed(1));
        let mut second = Burst::new(seed(2));
        assert_ne!(phases(&mut first, 2000), phases(&mut second, 2000));
    }

    #[test]
    fn the_settings_round_trip() {
        let settings = Settings::read(stored(&[]));
        assert_eq!(
            settings,
            Settings {
                sprint: DEFAULT_SPRINT,
                breath: DEFAULT_BREATH,
                family: true,
            }
        );
        assert_eq!(settings.factor(true), SPRINT_FACTORS[DEFAULT_SPRINT]);
        assert_eq!(settings.factor(false), BREATH_FACTORS[DEFAULT_BREATH]);
        assert_eq!(
            settings.labels(),
            [
                format!("Run speed: x{:.1} (12 to 20 s)", SPRINT_FACTORS[DEFAULT_SPRINT]),
                format!(
                    "Walking speed: x{:.1} (4 to 8 s)",
                    BREATH_FACTORS[DEFAULT_BREATH]
                ),
                "Zombies: zombie, husk, drowned, zombie villager".to_owned(),
            ]
        );
        let mut settings = Settings::read(stored(&[
            (SETTING_SPRINT, "4\n"),
            (SETTING_BREATH, "3"),
            (SETTING_FAMILY, "family"),
        ]));
        assert_eq!(settings.factor(true), 3.5);
        assert_eq!(settings.factor(false), 0.75);
        assert!(settings.drives("minecraft:husk"));
        // A value out of range falls back to the default.
        let mut guarded = Settings::read(stored(&[(SETTING_SPRINT, "9")]));
        assert_eq!(guarded.sprint, DEFAULT_SPRINT);
        // Small buttons wrap around and are kept.
        settings.click(0);
        settings.click(1);
        settings.click(2);
        assert_eq!(
            settings.stored(),
            [
                (SETTING_SPRINT, "0".to_owned()),
                (SETTING_BREATH, "0".to_owned()),
                (SETTING_FAMILY, "zombie".to_owned()),
            ]
        );
        guarded.click(9);
        assert_eq!(guarded.sprint, DEFAULT_SPRINT);
    }

    #[test]
    fn the_whole_family_is_driven_unless_plain_zombies_are_chosen() {
        // Nothing stored: the whole family, husk, drowned and zombie villager too.
        let family = Settings::read(stored(&[]));
        for kind in FAMILY {
            assert!(family.drives(kind), "{kind}");
        }
        for other in [
            "minecraft:skeleton",
            "minecraft:zombified_piglin",
            "minecraft:zoglin",
            "minecraft:player",
        ] {
            assert!(!family.drives(other), "{other}");
        }
        // The button picks plain zombies alone, and back.
        let mut plain = Settings::read(stored(&[(SETTING_FAMILY, "zombie")]));
        assert!(plain.drives("minecraft:zombie"));
        for kind in FAMILY[1..].iter() {
            assert!(!plain.drives(kind), "{kind}");
        }
        plain.click(2);
        for kind in FAMILY {
            assert!(plain.drives(kind), "{kind}");
        }
    }

    #[test]
    fn the_registry_drives_the_mobs_it_ticked() {
        let settings = Settings::read(stored(&[(SETTING_FAMILY, "family")]));
        let mut bursts = Bursts::default();
        assert_eq!(bursts.factor(21, &settings), None);
        let sprinting = bursts.tick(21);
        assert_eq!(bursts.factor(21, &settings), Some(settings.factor(sprinting)));
        // Another mob keeps its own cycle.
        assert_eq!(bursts.factor(22, &settings), None);
        bursts.forget(21);
        assert_eq!(bursts.factor(21, &settings), None);
        bursts.tick(31);
        bursts.clear();
        assert_eq!(bursts.factor(31, &settings), None);
    }

    #[test]
    fn mobs_that_stop_ticking_are_forgotten() {
        let settings = Settings::read(stored(&[]));
        let mut bursts = Bursts::default();
        for _ in 0..FORGET_TICKS {
            bursts.tick(5);
        }
        assert!(bursts.factor(5, &settings).is_some());
        for _ in 0..FORGET_TICKS {
            bursts.tick(6);
        }
        bursts.sweep();
        assert_eq!(bursts.factor(5, &settings), None, "5 stopped ticking");
        assert!(bursts.factor(6, &settings).is_some(), "6 is still ticking");
    }
}
