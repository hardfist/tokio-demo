use std::time::{Duration, Instant};

use tokio::spawn;
fn blocking_cpu_task(){
    
}
fn blocking_io_task(){
    std::thread::sleep(Duration::from_secs(1));
}

#[tokio::main(worker_threads =2)]
async fn main(){
    let start = Instant::now();
    let t1 =spawn(async {
        blocking_io_task();
    });
    let t2 = spawn(async {
       blocking_io_task(); 
    });
    let t3 = spawn(async {
        blocking_io_task();
    });
    let _t = tokio::join!(t1,t2,t3);
    let duration = start.elapsed().as_secs();
    println!("secs: {duration}");
}