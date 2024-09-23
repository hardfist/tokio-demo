use std::time::{Duration, Instant};

use tokio::spawn;
fn blocking_cpu_task() {}
fn blocking_io_task() {
    std::thread::sleep(Duration::from_secs(10));
}

#[tokio::main(worker_threads = 2)]
async fn main() -> anyhow::Result<()> {
    console_subscriber::init();
    let start = Instant::now();
    let t1 = tokio::task::Builder::new().name("t1").spawn(async {
        blocking_io_task();
    })?;
    let t2 = tokio::task::Builder::new().name("t2").spawn(async {
        blocking_io_task();
    })?;
    let t3 = tokio::task::Builder::new().name("t3").spawn(async {
        blocking_io_task();
    })?;
    let _t = tokio::join!(t1, t2, t3);
    let duration = start.elapsed().as_secs();
    println!("secs: {duration}");
    Ok(())
}
