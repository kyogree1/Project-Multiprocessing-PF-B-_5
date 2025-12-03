use std::process::{Command, Child};
use std::{thread, time::Duration};

const NUM_WORKERS: usize = 4;

fn spawn_worker(id: usize) -> Child {
    println!("[LAUNCHER] Spawn worker process #{id}");

    Command::new("target/debug/worker.exe")
        .env("WORKER_ID", id.to_string())
        .spawn()
        .expect("Gagal spawn worker")
}

fn main() {
    println!("[LAUNCHER] Multiprocess Worker Manager aktif...");
    println!("[LAUNCHER] Menjalankan {NUM_WORKERS} worker processes...");

    let mut workers: Vec<Child> = Vec::new();

    // Spawn initial workers
    for i in 0..NUM_WORKERS {
        workers.push(spawn_worker(i + 1));
    }

    loop {
        // Restart worker jika mati
        for (i, child) in workers.iter_mut().enumerate() {
            match child.try_wait() {
                Ok(Some(status)) => {
                    println!("[LAUNCHER] Worker {} mati (status: {}). Restarting...",
                             i + 1, status);
                    *child = spawn_worker(i + 1);
                }
                _ => {}
            }
        }

        thread::sleep(Duration::from_secs(2));
    }
}
