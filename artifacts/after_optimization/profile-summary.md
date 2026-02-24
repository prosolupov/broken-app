# Отчёт По Профилированию (Шаг 4)

## Сценарий
- Бинарь бенча `baseline`: `target/release/deps/baseline-ff05fc79cb85d481`

## perf + flamegraph
Использованные команды:
- `perf record -F 997 -g --call-graph dwarf -o artifacts/step4/perf-baseline.data -- <baseline-bin>`
- `flamegraph --perfdata artifacts/step4/perf-baseline.data -o artifacts/step4/perf-flamegraph.svg`

Основные CPU-узкие места по `perf report`:
- `broken_app::algo::slow_fib` — **38.13%**
- `broken_app::algo::slow_dedup` — **35.34%**
- `core::slice::sort::unstable::ipnsort` — **22.73%**
- `broken_app::sum_even` — **1.16%**

Вывод:
- Основная CPU-нагрузка сосредоточена в рекурсии `slow_fib` и в сортировке внутри `slow_dedup`.
- `sum_even` не является значимым потребителем CPU.

Примечание по символам ядра:
- В системе включён `kptr_restrict`, поэтому часть сэмплов ядра отображается как неразрешённые (`[unknown] [k]`).

## Артефакты
- `artifacts/step4/perf-baseline.data`
- `artifacts/step4/perf-report.txt`
- `artifacts/step4/perf-flamegraph.svg`
- `artifacts/step4/perf-flamegraph.png`
