use std::time::{Duration, Instant};

use tokio::{spawn, task::block_in_place};
fn blocking_cpu_task() {}
fn blocking_io_task(i:i32)-> i32 {
    std::thread::sleep(Duration::from_secs(1));
    i
}

#[tokio::main(worker_threads = 1)]
async fn main() {
    let start = Instant::now();
    let t0 = async {
        blocking_io_task(0)
    };
    let t1 = spawn(async {
        block_in_place(|| {
            blocking_io_task(1)
        })
    });
    let t2 = spawn(async {
        block_in_place(|| {
            blocking_io_task(2)
        })
    });
    let t3 = spawn(async {
        block_in_place(|| {
            blocking_io_task(3)
        })
    });
    let t = tokio::join!(t1, t2, t3,t0);
    dbg!(t);
    let duration = start.elapsed().as_secs();
    println!("secs: {duration}");
}
