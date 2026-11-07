# Страница мода в реестре (данные для playhorizon.dev/mods/submit)

Заполните эти поля в форме отправки. Сам файл мода — `zombies_are_running-1.1.0.hnmod`
(он же `horizon-mods/native/target/zombies_are_running-1.1.0.hnmod`).

| Поле формы | Значение |
| --- | --- |
| Вид | native mod (`.hnmod`) |
| Файл | `zombies_are_running-1.1.0.hnmod` |
| id | `zombies_are_running` |
| Версия | `1.1.0` |
| Версия Minecraft | `26.3` |
| SHA-256 файла | `0c4ada349a4aca052fd90e23717f3f9a33fd255cf2949a1f3dbede325bd973e1` (9445 байт) |
| Название | Zombies Are Running |
| Автор (authors) | **Bogdan_Tokarev** |
| Лицензия | MIT |
| Иконка | `icon.png` — 16×16 пиксель-арт, 1 пиксель = 1 px (арт заказчика, восстановлен из `icon-512.png` без потерь) |
| Исходники | публичный репозиторий с папкой `zombies-are-running/` (структура описана в `README.md`) |

**Краткое описание (одна строка):**
Zombies chase you in bursts: 12–20 s of running, then 4–8 s of walking to catch their breath. By Bogdan_Tokarev.

**Описание (Markdown):**

```markdown
Zombies Are Running changes how zombies chase you. A zombie that has a target does it in bursts:

- **Sprint — 12 to 20 seconds.** It runs at its prey: movement speed ×2.0.
- **Cooldown — 4 to 8 seconds.** It catches its breath and walks after its prey: movement speed ×0.5.
- Then it runs again, and so on, for as long as it has something to chase.

Each zombie draws its own lengths from those ranges, so a crowd never runs and rests in step:
there is always one running at you and always a moment to get away.

What it does and what it doesn't:

- It drives the whole zombie family by default: **zombie, husk, drowned and zombie villager**
  (`minecraft:zombie`, `minecraft:husk`, `minecraft:drowned`, `minecraft:zombie_villager`). Skeletons,
  zombified piglins, zoglins, giants and players are never touched; the *Zombies* button in Settings
  can narrow it to plain zombies.
- Only a zombie **with a target** changes speed. An idle zombie walks exactly as the game made it,
  and every other mob is untouched.
- It runs on the server side (single player, and the server thread of a LAN world), so every
  player sees the same zombies.
- **Options → Mods** has the mod's switch; **Settings** picks the two speeds (×1.5 to ×3.5 while
  running, ×0.4 to ×0.75 while walking) and whether it drives the whole family or plain zombies alone.
- It adds no blocks, items or assets, sends nothing over the network of its own, and keeps its
  three settings in `horizon-mods/.horizon-mod-data/zombies_are_running/`.

By Bogdan_Tokarev. MIT.
```

**Что менялось в версии 1.1.0 (поле «what changed»):**

```text
1.1.0 — the mod is called Zombies Are Running. The burst cycle is per zombie (12–20 s of running,
4–8 s of walking), its lengths are drawn by each zombie, a target is required before a zombie's
speed changes at all, the whole zombie family (zombie, husk, drowned, zombie villager) is driven by
default, and the settings page adds the walking speed and the family button.
```
