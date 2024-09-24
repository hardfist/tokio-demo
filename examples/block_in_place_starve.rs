use std::{ time::{Duration, Instant}};

use tokio::{spawn, task::{block_in_place, spawn_blocking}};
fn blocking_cpu_task() {}
fn blocking_io_task(i:i32)-> i32 {
    println!("started:{}",i);
    std::thread::sleep(Duration::from_secs(1));
    i
}
#[tokio::main(worker_threads = 2)]
async fn main() {
    let start = Instant::now();
    let t1 = spawn(async {
        // this will block the following async task
        let t = async {
          blocking_io_task(0)
        };
        block_in_place(|| {
            blocking_io_task(1)
        });
        t.await;
        // let x = spawn_blocking(|| {
        //     blocking_io_task(1)
        // });
        // tokio::join!(x,t)
        1
       
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
    let t = tokio::join!(t1, t2, t3);
    dbg!(t);
    let duration = start.elapsed().as_secs();
    println!("secs: {duration}");
}
