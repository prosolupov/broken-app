# Бенчмарки До Оптимизации

## Команда запуска
`cargo bench --bench criterion -- --save-baseline before_opt`

## Покрытые горячие пути
- `sum_even` на `50_000` и `1_000_000` элементов (большой вход)
- `slow_fib` на `n=28` и `n=32`
- `slow_dedup` на `5_000` и `20_000` уникальных значений (длина входа удвоена из-за дублей)

## Выходные артефакты
- Лог консоли: `artifacts/before_optimization/benchmark_before.log`
- Полные отчёты Criterion (HTML/SVG/JSON): `artifacts/before_optimization/criterion_before/`
- Главная страница отчёта: `artifacts/before_optimization/criterion_before/report/index.html`

## Ключевые результаты (из лога)
- `sum_even/50000`: ~22.3 us
- `sum_even/1000000`: ~0.85-0.95 ms
- `slow_fib/28`: ~1.51 ms
- `slow_fib/32`: ~10.4 ms
- `slow_dedup/5000`: ~15.7 ms
- `slow_dedup/20000`: ~251-253 ms
