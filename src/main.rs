use std::sync::atomic::AtomicI64;
use std::sync::atomic::Ordering::SeqCst;


fn increment(a: &AtomicI64) {
    let mut current = a.load(SeqCst);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, SeqCst, SeqCst) {
            Ok(_) => return,
            Err(v) => current = v,
        }
    }
}

fn atomic_bench1() {
    let a = &AtomicI64::new(0);
        std::thread::scope(|s| {
           for _ in 0..8 {
               s.spawn(|| {
                   for _ in 0..10_000_000 {
                       a.fetch_add(1, SeqCst);
                   }
               });
           }
        });
    dbg!(a);
}

fn atomic_bench2() {
    let a = &AtomicI64::new(0);
    std::thread::scope(|s| {
        for _ in 0..8 {
            s.spawn(move || {
                for _ in 0..10_000_000 {
                    increment(&a);
                }
            });
        }
    });
    dbg!(a);
}

fn channel_bench() {
    let mut a = 0_i64;
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::scope(|s| {
        for _ in 0..8 {
            let tx_th = tx.clone();
            let mut a_th = a.clone();
            s.spawn(move || {
                for _ in 0..10_000_000 {
                    a_th += 1;
                }
                tx_th.send(a_th).unwrap();
            });
        }
    });
    drop(tx);
    for r in rx {
        a += r;
    }
    dbg!(a);
}

fn normal_bench() {
    let mut a = 0_i64;
    for _ in 0..80_000_000 {
        a += 1
    }
    dbg!(a);
}

fn main() {
    let start = std::time::SystemTime::now();
    atomic_bench1();
    let finish = std::time::SystemTime::now();
    let work_time = finish.duration_since(start).unwrap();
    println!("Work time (atomic+=) - {:?}", work_time);

    let start = std::time::SystemTime::now();
    atomic_bench2();
    let finish = std::time::SystemTime::now();
    let work_time = finish.duration_since(start).unwrap();
    println!("Work time (atomic==) - {:?}", work_time);

    let start = std::time::SystemTime::now();
    channel_bench();
    let finish = std::time::SystemTime::now();
    let work_time = finish.duration_since(start).unwrap();
    println!("Work time (channel) - {:?}", work_time);

    let start = std::time::SystemTime::now();
    normal_bench();
    let finish = std::time::SystemTime::now();
    let work_time = finish.duration_since(start).unwrap();
    println!("Work time (normal) - {:?}", work_time);

    // atomic_bench1   ~1200ms (DEV) - ~1130ms (REL)
    // atomic_bench2   ~7700ms (DEV) - ~5500ms (REL)
    // channel_bench    ~125ms (DEV) -    ~1ms (REL)
    // normal_bench     ~600ms (DEV) -  ~0.6ms (REL)
}