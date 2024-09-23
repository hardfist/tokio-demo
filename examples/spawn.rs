use std::time::{Duration, Instant};

use tokio::{runtime::Handle, spawn, task::block_in_place};
fn blocking_cpu_task() {}
fn blocking_io_task() {
    std::thread::sleep(Duration::from_secs(10));
}

#[tokio::main(worker_threads = 2)]
async fn main() -> anyhow::Result<()> {
    console_subscriber::init();
    // tokio::spawn(
        
    //     async move {
    //         let monitor = tokio_metrics::TaskMonitor::new();
    //          let frequency = std::time::Duration::from_millis(500);
    //          for metrics in monitor.intervals() {
    //             println!("{:?}", metrics);
    //             tokio::time::sleep(frequency).await;
    //         }
    //     }
    // );
    
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
