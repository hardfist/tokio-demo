use std::time::{Duration, Instant};

use tokio::{spawn, task::block_in_place};
fn blocking_cpu_task() {}
fn blocking_io_task() {
    std::thread::sleep(Duration::from_secs(1));
}

#[tokio::main(worker_threads = 2)]
async fn main() {
    let start = Instant::now();
    let t1 = spawn(async {
        block_in_place(|| {
            blocking_io_task();
        })
    });
    let t2 = spawn(async {
        block_in_place(|| {
            blocking_io_task();
        })
    });
    let t3 = spawn(async {
        block_in_place(|| {
            blocking_io_task();
        })
    });
    let _t = tokio::join!(t1, t2, t3);
    let duration = start.elapsed().as_secs();
    println!("secs: {duration}");
}
