# Rayon Demo

Second additional task for the Parallel Programming class. Shows how effortlessly [rayon](https://github.com/rayon-rs/rayon) speedups the workloads.

Launch with:
```bash
cargo run --release
```

Execution result (CPU with 8 cores):
```
Генерування 10000000 випадкових чисел...
Початок тестування...

Послідовно: знайдено 472801 простих чисел за 34.234398653s
Паралельно: знайдено 472801 простих чисел за 4.742153237s

Прискорення: 7.22x
```

