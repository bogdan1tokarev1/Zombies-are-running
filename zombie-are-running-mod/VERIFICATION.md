# Как проверялся мод Zombies Are Running 1.1.0

Всё ниже прогнано на реальном коде Horizon (Nightly `2ebf00a`, 2026-11-07) и на
настоящем `horizon-mixins` 0.6.0 (`e8cd0aa` — та же версия, на которой строятся
моды самой Horizon). Проверка сделана заново после переименования мода и смены
иконки.

## 1. Логика мода — юнит-тесты

```sh
cd zombies-are-running && cargo test
```

```
running 7 tests
test tests::a_zombie_starts_by_catching_its_breath ... ok
test tests::neighbours_do_not_share_a_cycle ... ok
test tests::the_whole_family_is_driven_unless_plain_zombies_are_chosen ... ok
test tests::mobs_that_stop_ticking_are_forgotten ... ok
test tests::the_registry_drives_the_mobs_it_ticked ... ok
test tests::the_settings_round_trip ... ok
test tests::runs_and_breaths_stay_in_the_documented_ranges ... ok
test result: ok. 7 passed; 0 failed
```

Что проверяют: каждая фаза укладывается в 12–20 с (бег) и 4–8 с (шаг); зомби
начинает с шага; соседние зомби не идут в ногу; настройки читаются и
записываются как файлы; **по умолчанию ускоряется всё семейство (зомби, кадавр,
утопленник, зомби-крестьянин), а скелет, зомбифицированный пиглин, зоглин и
игрок — никогда, и кнопка в Settings сужает охват до обычного зомби и обратно**;
зомби, которые перестали тикать, через минуту забываются.

`cargo clippy --all-targets` — без предупреждений (и библиотека, и пример).

## 2. Цикл в работе (модель, 6 зомби, 120 секунд)

```sh
cargo run --example cycle
```

`R` — бежит за игроком, `.` — идёт спокойным шагом, один символ = 1 секунда:

```
Zombies Are Running 1.1.0 — 12 to 20 s of running, 4 to 8 s of walking
zombie 1 |.....RRRRRRRRRRRRRRRRR......RRRRRRRRRRRRRRRRRRR.....RRRRRRRRRRRRRRRRR.......RRRRRRRRRRRRRRRRRRRR.....RRRRRRRRRRRRRRRRRRR|
zombie 2 |......RRRRRRRRRRRRRRRRRRRR.......RRRRRRRRRRRRRRRRR.....RRRRRRRRRRRRRRRR.......RRRRRRRRRRRRRRRR.......RRRRRRRRRRRRRRRRRRR|
zombie 3 |......RRRRRRRRRRRRRRR....RRRRRRRRRRRRRRRRRRRR.....RRRRRRRRRRRRRRRRRR.......RRRRRRRRRRRRRRRR......RRRRRRRRRRRRRRR........RRRR|
zombie 4 |......RRRRRRRRRRRRRRRRR......RRRRRRRRRRRRR.......RRRRRRRRRRRRRRRRRR........RRRRRRRRRRRRRRR......RRRRRRRRRRRRRRRR.....RRRRR|
zombie 5 |......RRRRRRRRRRRRRR........RRRRRRRRRRRRRRRRRRRR....RRRRRRRRRRRRRRRRRRR......RRRRRRRRRRRRRRRRRR.....RRRRRRRRRRRRRRRR....|
zombie 6 |....RRRRRRRRRRRRR.......RRRRRRRRRRRRRRRRRRRR....RRRRRRRRRRRRRRRRRRR....RRRRRRRRRRRRRRR....RRRRRRRRRRRRRRR......RRRRRRRRR|
```

## 3. Иконка

Оба присланных файла — один и тот же рисунок: `icon-512.png` (512×512, блок
32 px) и `icon (23).png` (64×64, блок 4 px). Взял 512-й как источник и уменьшил
`NEAREST` ровно в 32 раза: получился точный оригинальный арт **16×16, 14
цветов, 312 байта**, `icon.png` (1 пиксель рисунка = 1 px файла, без сглаживания
и без потерь). Проверено программно, что оба файла дают побайтово одинаковый
16×16 арт.

## 4. Сборка бандла в игру (horizon-mixins)

```sh
horizon-mixins apply --workspace <исходники Horizon 2ebf00a> \
                     --mods <бандл мода> --out <stage>
```

```
{ "bridges": 0, "crates": 0, "data": 0, "injections": 6, "modules": 1,
  "post_build": 0, "stage_changed": true, "states": 0, "status": "ok" }
```

После переименования в исходники игры встаёт новое имя:
`pub mod zombies_are_running { … }` в `crates/horizon-client/src/client_lib.rs`,
и все шесть инъекций на месте.

Первая проверка (до переименования) поймала настоящую ошибку, которую не видно
по документации: `Mob` живёт не в крейте `horizon-client`, а в
`horizon-client-lib` (`crates/horizon-client/src/mobs.rs` — корень библиотеки
`client_lib.rs`). Поэтому модуль мода добавляется в `client_lib.rs` как
`pub mod zombies_are_running` (его видят оба крейта: `crate::zombies_are_running`
из библиотеки и `horizon_client_lib::zombies_are_running` из игры — состояние
одно), а RETURN-хук целится в
`horizon_client_lib::mobs::Mob::movement_speed`.

## 5. Компиляция модифицированной игры

```sh
cd <stage> && RUSTFLAGS="-D warnings" cargo check --locked \
    -p horizon-client -p horizon-client-lib
cd <stage> && RUSTFLAGS="-D warnings" cargo check --locked -p horizon-client --lib
```

```
    Checking horizon-client-lib v0.1.0 (…/crates/horizon-client-lib)
    Checking horizon-client v0.1.0 (…/crates/horizon-client)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 40.34s
```

`-D warnings` — как в самой `native.py` (любое предупреждение было бы ошибкой).
Собираются обе части: модуль `zombies_are_running` и RETURN-хук в
`horizon-client-lib`, пять helper'ов в `horizon-client`. Мод встал в нужные
места исходников игры:

```
mixins/zombies_are_running/listen.rs   → Game::native_entity_tick_listened
mixins/zombies_are_running/tick.rs     → Game::native_entity_tick_pre
mixins/zombies_are_running/list.rs     → Game::native_mods
mixins/zombies_are_running/settings.rs → Game::native_settings
mixins/zombies_are_running/click.rs    → Game::native_settings_click
mixins/zombies_are_running/speed.rs    → horizon_client_lib::mobs::Mob::movement_speed (RETURN)
runtime.rs                             → pub mod zombies_are_running в client_lib.rs
```

## 6. Пакет `.hnmod`

```sh
python3 native/tools/native.py package zombies-are-running
```

```json
{ "file": "native/target/zombies_are_running-1.1.0.hnmod",
  "size": 9445,
  "sha256": "0c4ada349a4aca052fd90e23717f3f9a33fd255cf2949a1f3dbede325bd973e1" }
```

Внутри: `horizon-native.json` (id `zombies_are_running`, версия `1.1.0`, игра
`26.3`, hooks 146, коммиты Horizon и horizon-mixins), все `.rs` и `.toml`
бандла, `LICENSE`. Сборка детерминированная: те же файлы дают тот же SHA-256.

## 7. Что проверить нельзя было здесь — и почему

**Запустить игру в этом окружении невозможно**, и это не связано с модом:

* Horizon 26.3 — отдельный клиент, которому нужны подготовленные ресурсы
  установленного Minecraft 26.3 (`horizon-prepare`), лаунчер и GPU с окном.
  В песочнице нет ни установленного Minecraft, ни лаунчера, ни дисплея.
* `native.py proof` (запуск собранной игры через `horizon-ctl --offscreen`, как
  это делают моды Horizon в CI) требует `--resources` — подготовленные ресурсы
  26.3, в репозитории их нет («none are in this repository»).

Поэтому «нет ли краша» проверено на уровне компиляции (пункт 5) и логики
(пункты 1–2), а не прогоном мира. Как проверить за минуту у себя:

```sh
git clone https://github.com/horizon-rust/horizon-mods
# положить эту папку как native/zombies-are-running
cd horizon-mods
python3 native/tools/native.py check zombies-are-running   # apply + cargo check
horizon-launcher native-build native/zombies-are-running --trust-native-code
```

и в лаунчере нажать **Play**: в мире `/summon minecraft:zombie` (в креативе),
отойти на 10 блоков и смотреть — 12–20 секунд зомби должен догонять вас бегом,
4–8 секунд идти спокойным шагом, потом снова бежать. В **Options → Mods**
строка «Zombies Are Running 1.1.0» с этой иконкой и её **Settings**.
