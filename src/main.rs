use std::sync::{Mutex, Arc};
use std::thread;
use std::cmp::{min, max};
use std::time::Duration;
use rand::{thread_rng, Rng};

const HUMANS: usize = 5;

fn main() {
    let mut forks = Vec::with_capacity(HUMANS);
    for _ in 0..HUMANS {
        forks.push(Mutex::new(false));
    }
    let mut forks = Arc::new(forks);
    let mut philosophers = vec![];
    for i in 0..=4 {
        let mut forks = forks.clone();
        let philosopher = thread::spawn(move || {
            let numer = i.clone();
            loop {
                let mut thread_rng = thread_rng();
                let left_fork_index = min(numer, (numer + 1) % HUMANS);
                let right_fork_index = max(numer, (numer + 1) % HUMANS);
                // println!("Philosopher numer {} wanted fork numer {}", numer , left_fork_index);
                let mut left_fork = forks[left_fork_index].lock().unwrap();
                // println!("Philosopher numer {} wanted fork numer {}", numer , right_fork_index);
                let mut right_fork = forks[right_fork_index].lock().unwrap();

                println!("{} Eating. Philosopher {} enjoys using forks {} {}", numer, numer, left_fork_index, right_fork_index);
                thread::sleep(Duration::from_millis(thread_rng.gen_range(1000, 3000)));
                println!("{} Sleeping. Philosopher {} stops eating using forks {} {}", numer, numer, left_fork_index, right_fork_index);
                std::mem::drop(left_fork);
                std::mem::drop(right_fork);
                thread::sleep(Duration::from_millis(thread_rng.gen_range(1000, 3000)));
            }
        });
        philosophers.push(philosopher);
    }
    for philosopher in philosophers {
        philosopher.join();
    }
}