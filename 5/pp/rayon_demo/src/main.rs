use rayon::prelude::*;
use std::time::Instant;

// Функція для перевірки числа на простоту (навантаження CPU)
fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(f64::from(n).sqrt() as u32) {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}

fn main() {
    // Підготовка даних
    let data_size = 10_000_000;
    println!("Генерування {data_size} випадкових чисел...");

    // Тут теж використано Rayon для швидкої генерації
    let numbers: Vec<u32> = (0..data_size)
        .into_par_iter()
        .map(|_| rand::random::<u32>())
        .collect();

    println!("Початок тестування...\n");

    // Послідовне виконання
    let start_seq = Instant::now();
    let count_seq = numbers.iter().filter(|&&x| is_prime(x)).count();
    let duration_seq = start_seq.elapsed();

    println!("Послідовно: знайдено {count_seq} простих чисел за {duration_seq:?}");

    // Паралельне виконання з Rayon
    let start_par = Instant::now();
    let count_par = numbers
        .par_iter() // Вся магія тут
        .filter(|&&x| is_prime(x))
        .count();
    let duration_par = start_par.elapsed();

    println!("Паралельно: знайдено {count_par} простих чисел за {duration_par:?}");

    // Висновок
    let speedup = duration_seq.as_secs_f64() / duration_par.as_secs_f64();
    println!("\nПрискорення: {speedup:.2}x");
}
