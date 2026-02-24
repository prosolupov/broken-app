# Шаг 7: Проверка После Оптимизаций

## Проверки корректности

Выполнены повторные прогоны после оптимизаций:
- `cargo test --tests`
- `cargo +nightly miri test`
- `valgrind --leak-check=full cargo test --tests`
- `valgrind --leak-check=full --show-leak-kinds=all <integration-binary>`
- `ASAN_OPTIONS=detect_leaks=0 RUSTFLAGS='-Zsanitizer=address' cargo +nightly test --test integration -Zbuild-std --target x86_64-unknown-linux-gnu`
- `RUSTFLAGS='-Zsanitizer=thread' cargo +nightly test --test integration -Zbuild-std --target x86_64-unknown-linux-gnu`

Все прогоны завершились успешно.

Логи:
- `artifacts/after_optimization/post_cargo_test.log`
- `artifacts/after_optimization/post_miri_test.log`
- `artifacts/after_optimization/post_valgrind_tests.log`
- `artifacts/after_optimization/post_valgrind_integration_bin.log`
- `artifacts/after_optimization/post_asan_integration.log`
- `artifacts/after_optimization/post_tsan_integration.log`

## Бенчмарки после оптимизаций

Запуск:
- `cargo bench --bench criterion -- --baseline before_opt`

Артефакты:
- Лог: `artifacts/after_optimization/benchmark_after.log`
- Отчёты criterion: `artifacts/after_optimization/criterion_after/`
- Сравнение с baseline: встроено в вывод `criterion` (`change:`)

## Сравнение before vs after

Ниже сравнение по медианам из логов `benchmark_before.log` и `benchmark_after.log`.

- `sum_even/50000`: 22.340 us -> 22.025 us, ускорение ~`1.01x` (~1.4%)
- `sum_even/1000000`: 897.66 us -> 910.44 us, в пределах шума (`criterion`: `No change in performance detected`)
- `slow_fib/28`: 1.5136 ms -> 31.696 ns, ускорение ~`47_754x`
- `slow_fib/32`: 10.424 ms -> 36.145 ns, ускорение ~`288_394x`
- `slow_dedup/5000`: 15.731 ms -> 23.227 us, ускорение ~`677x`
- `slow_dedup/20000`: 252.17 ms -> 94.206 us, ускорение ~`2_677x`

## Изменения по аллокациям и копированию

- `slow_dedup`:
  - было: многократные сортировки на каждой вставке + многократные проходы
  - стало: одна копия входа, одна сортировка, `dedup()` in-place
  - эффект: резко меньше лишней работы CPU и меньше промежуточных операций над буфером

- `slow_fib`:
  - было: экспоненциальная рекурсия (огромное число повторных вызовов)
  - стало: линейный итеративный расчёт без рекурсивного дерева вызовов
  - эффект: порядок сложности снижен с экспоненциальной до линейной
