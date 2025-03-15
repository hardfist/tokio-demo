use std::time::Duration;

use tokio::{runtime::{Builder, Runtime}, task::{coop::unconstrained, spawn_blocking}, time::sleep};
use pollster::block_on;
fn main(){
    loop3();
}
fn loop1(){
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        block_on(async {
            let mut cnt = 0;
            loop {
                sleep(Duration::from_millis(10)).await;
                cnt += 1;
                println!("{} ", cnt);
            }
        })
    });
}
fn loop2 (){
    let rt = Runtime::new().unwrap();
    rt.block_on(unconstrained( async {
        block_on(async {
            let mut cnt = 0;
            loop {
                sleep(Duration::from_millis(10)).await;
                cnt += 1;
                println!("{} ", cnt);
            }
        })
    }));
}
fn loop3 (){
    let rt =Builder::new_multi_thread().disable_lifo_slot().enable_all().build().unwrap();
    rt.block_on(async {
        block_on(async {
            let mut cnt = 0;
            loop {
                sleep(Duration::from_millis(10)).await;
                cnt += 1;
                println!("{} ", cnt);
            }
        })
    });
}