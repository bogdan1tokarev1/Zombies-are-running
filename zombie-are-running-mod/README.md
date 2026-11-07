# Zombies Are Running

**A Horizon native mod for Minecraft 26.3 — version 1.1.0 — by Bogdan_Tokarev.**

Zombies no longer run at you forever. A zombie that is chasing a player does it in bursts:

| Phase | How long | What it does |
| --- | --- | --- |
| Sprint | **12 to 20 seconds** | runs at its prey: movement speed ×2.0 |
| Cooldown | **4 to 8 seconds** | catches its breath: walks after the prey, movement speed ×0.5 |

Then it sprints again, and so on, for as long as it has something to chase. The lengths are
drawn per zombie, so a crowd of them never runs and rests in step — some are always slow, and
there is always a moment to get away.

* It only changes a zombie's speed while that zombie has a target: an idle zombie walks as the
  game made it, and every other mob is untouched.
* Works in single player and on the server side of a LAN world (the mod ticks on the server, so
  all players see the same zombies).
* Turn it off and on in **Options → Mods**, and change the two speeds there (**Settings**).

Its icon (`icon.png`, 16×16 pixel art, one art pixel per file pixel) shows a zombie running with
motion streaks, by Bogdan_Tokarev.

## Which mobs it drives

By default the **whole zombie family** runs in bursts:

| Mob | id |
| --- | --- |
| zombie | `minecraft:zombie` |
| husk | `minecraft:husk` |
| drowned | `minecraft:drowned` |
| zombie villager | `minecraft:zombie_villager` |

The **Settings** button *Zombies* narrows it to plain `minecraft:zombie` and back. Nothing else is
touched: skeletons, zombified piglins (`minecraft:zombified_piglin`), zoglins (`minecraft:zoglin`),
giants and players keep their own speed.

## Install

1. In the Horizon launcher, **Mods**, find Zombies Are Running, **Install** into the instance you play.
2. Press **Build** on the *Native build of {instance}* row, read the red warning and confirm
   with **I trust these mods: build** (the launcher compiles the game with the mod; the first
   build downloads Rust and takes 20 to 30 minutes).
3. Play that instance. In the game the mod is listed under **Options → Mods**.

Or from a file: install `zombies_are_running-1.1.0.hnmod` from the Mods page (it is not sandboxed: it is
Rust compiled into your game, so install only mods whose source you trust).

## Settings (Options → Mods → Zombies Are Running → Settings)

| Button | Values |
| --- | --- |
| Run speed | ×1.5, **×2.0**, ×2.5, ×3.0, ×3.5 |
| Walking speed | ×0.4, **×0.5**, ×0.6, ×0.75 |
| Zombies | **zombie, husk, drowned, zombie villager**, or zombie alone |

The settings live in `horizon-mods/.horizon-mod-data/zombies_are_running/` (one file per setting:
`sprint`, `breath`, `family`).

## How it is built

A [horizon-mixins](https://github.com/horizon-rust/horizon-mixins) bundle (`mixins/zombies_are_running/`)
that the Horizon launcher compiles into the game.

| File | What it is |
| --- | --- |
| `mixins/zombies_are_running/horizon-mixin.toml` | The bundle: one module, five HEAD hooks and one RETURN hook |
| `mixins/zombies_are_running/runtime.rs` | The cycles, the settings and the driven mobs — added to the game as `pub mod zombies_are_running` in `client_lib.rs`; also this crate's library, for its tests |
| `mixins/zombies_are_running/listen.rs` | Asks the server for entity ticks |
| `mixins/zombies_are_running/tick.rs` | One tick of every ticking entity: advances its cycle |
| `mixins/zombies_are_running/speed.rs` | `Mob::movement_speed` of a zombie that has a target: × the burst's factor |
| `mixins/zombies_are_running/list.rs`, `settings.rs`, `click.rs` | Its row and its settings page in Options → Mods |

The cycle itself is plain Rust: no game types, no dependencies, so `cargo test` runs it on its own
(7 tests: the lengths stay in 12–20 s and 4–8 s, phases alternate, neighbours differ, the
settings round-trip, only the chosen kinds are driven, and mobs that stop ticking are dropped).

Five hooks build on Horizon's stable **hook points** (`native_entity_tick_listened`,
`native_entity_tick_pre`, `native_mods`, `native_settings`, `native_settings_click`), and one is a
`RETURN` hook on the game's own `Mob::movement_speed`, which is where a zombie's movement speed is
read each tick. That second one names a function of Horizon's source, so the mod builds on the
commit it was tested on, `2ebf00a` (Nightly 2026-11-07, hook points API 411 — the bundle needs 146).

## Build and check

```sh
cargo test                                    # the logic
python3 native/tools/native.py check zombies-are-running                 # the pinned Horizon
python3 native/tools/native.py check zombies-are-running --horizon latest # the newest Nightly
python3 native/tools/native.py package zombies-are-running               # native/target/zombies_are_running-1.1.0.hnmod
```

(from a clone of [horizon-rust/horizon-mods](https://github.com/horizon-rust/horizon-mods), with
this folder as `native/zombies-are-running`).

License: MIT — see `LICENSE`. Author and maintainer: **Bogdan_Tokarev**.
