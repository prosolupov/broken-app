# broken-app

`broken-app` — проект про исправление типичных проблем в коде:
- логические ошибки,
- небезопасные места (`unsafe`/UB),
- проблемы конкурентности,
- медленные алгоритмы и их оптимизацию,
- проверку изменений через тесты, санитайзеры и бенчмарки.

Сейчас в репозитории лежит уже исправленная версия кода и артефакты сравнения производительности `before/after`.

## Структура

- `src/lib.rs` — публичные функции библиотеки:
  - `sum_even(&[i64]) -> i64` — сумма чётных чисел,
  - `leak_buffer(&[u8]) -> usize` — подсчёт ненулевых байтов,
  - `normalize(&str) -> String` — нормализация строки,
  - `average_positive(&[i64]) -> f64` — среднее по положительным,
  - `unsafe fn use_after_free() -> i32` — оставлена `unsafe` для совместимости API.
- `src/algo.rs` — алгоритмические задачи:
  - `slow_fib` (теперь итеративная линейная версия),
  - `slow_dedup` (сортировка + `dedup` in-place).
- `src/concurrency.rs` — потокобезопасный счётчик на `AtomicU64`.
- `src/bin/demo.rs` — демонстрационный бинарник.
- `tests/integration.rs` — интеграционные тесты основных сценариев.
- `benches/criterion.rs` и `benches/baseline.rs` — benchmarking.
- `artifacts/` — логи и отчёты до/после оптимизаций.
- `scripts/` — вспомогательные скрипты для сравнения и профилирования.

## Требования

- Rust toolchain (`rustup`, `cargo`).
- Для расширенной проверки:
  - nightly toolchain (`cargo +nightly`) для Miri/санитайзеров,
  - `valgrind` (Linux),
  - `perf` + `flamegraph` (Linux) для профилирования CPU.

## Быстрый старт

```bash
cargo run --bin demo
```

Ожидаемый вывод:

```text
sum_even: 6
non-zero bytes: 3
normalize: helloworld
fib(20): 6765
dedup: [1, 2, 3, 4]
```

## Тесты

```bash
cargo test
```

В проекте покрыты интеграционные проверки корректности по арифметике, нормализации, дедупликации, конкурентности и regression-тест для исторического `use_after_free` кейса.

## Бенчмарки

Запуск Criterion:

```bash
cargo bench --bench criterion
```

Сохранить baseline:

```bash
cargo bench --bench criterion -- --save-baseline before_opt
```

Сравнить с baseline:

```bash
cargo bench --bench criterion -- --baseline before_opt
```

Быстрый ручной бенч:

```bash
cargo bench --bench baseline
```

## Ключевые результаты оптимизаций

По артефактам в `artifacts/before_optimization` и `artifacts/after_optimization`:

- `slow_fib/32`: `10.424 ms -> 36.145 ns` (~`288_394x` быстрее),
- `slow_fib/28`: `1.5136 ms -> 31.696 ns` (~`47_754x` быстрее),
- `slow_dedup/20000`: `252.17 ms -> 94.206 us` (~`2_677x` быстрее),
- `slow_dedup/5000`: `15.731 ms -> 23.227 us` (~`677x` быстрее),
- `sum_even`: изменения небольшие и в пределах шума на крупном входе.

Подробности: `artifacts/after_optimization/post_verification_summary.md`.

## Дополнительная верификация (опционально)

Примеры команд (в основном для Linux CI/локального анализа):

```bash
cargo +nightly miri test
valgrind --leak-check=full cargo test --tests
ASAN_OPTIONS=detect_leaks=0 RUSTFLAGS='-Zsanitizer=address' cargo +nightly test --test integration -Zbuild-std --target x86_64-unknown-linux-gnu
RUSTFLAGS='-Zsanitizer=thread' cargo +nightly test --test integration -Zbuild-std --target x86_64-unknown-linux-gnu
```

## Профилирование

Скрипт `scripts/profile.sh` показывает базовый сценарий с `perf`:

```bash
cargo build --release
perf record -g ./target/release/demo
perf report
```

Также доступны сохранённые профили и flamegraph в `artifacts/after_optimization/`.
