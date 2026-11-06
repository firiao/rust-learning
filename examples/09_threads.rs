// 09. Потоки, каналы, Arc<Mutex<T>>.
//
// Запуск:
//   rustc --edition 2024 examples/09_threads.rs -o /tmp/threads
//   /tmp/threads
//
// Темы: thread::spawn, move-замыкания, join, mpsc, Arc<Mutex<T>>,
//        scoped threads, Send/Sync.

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 1. Простой поток и join.
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("поток: {i}");
            thread::sleep(Duration::from_millis(10));
        }
    });

    for i in 1..=2 {
        println!("main: {i}");
    }

    handle.join().unwrap();

    // 2. move-замыкание: владение данными переходит в поток.
    let data = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        println!("сумма в потоке: {sum}");
        sum
    });
    let sum = handle.join().unwrap();
    println!("поток вернул: {sum}");

    // 3. Канал mpsc: производитель → потребитель.
    let (sender, receiver) = std::sync::mpsc::channel();
    let producer = thread::spawn(move || {
        for i in 0..3 {
            sender.send(i).unwrap();
        }
    });

    for received in receiver {
        println!("получено: {received}");
    }
    producer.join().unwrap();

    // 4. Общее изменяемое состояние: Arc<Mutex<T>>.
    let counter = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::new();

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut value = counter.lock().unwrap();
            *value += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("counter = {}", *counter.lock().unwrap());

    // 5. Scoped threads: можно брать обычные ссылки, без 'static.
    let mut numbers = vec![1, 2, 3];
    thread::scope(|scope| {
        scope.spawn(|| {
            numbers.push(4);
        });
    });
    println!("numbers = {numbers:?}");

    // Упражнение: запусти 4 потока, каждый пусть считает сумму своего
    // квартала вектора; собери результаты через канал.
}
