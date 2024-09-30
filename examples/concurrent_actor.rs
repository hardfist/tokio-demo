use std::time::Duration;
use std::sync::Arc;
use tokio::join;
use tokio::sync::Mutex;

struct State {
    pub c1: u32,
    pub c2: u32
}
async fn act1(state: Arc<Mutex<State>>){
    while state.lock().await.c1 > 0 {
        tokio::time::sleep(Duration::from_secs(1)).await;
        state.lock().await.c1 -=1;
        if state.lock().await.c1 == 0 && state.lock().await.c2 == 0 {
            println!("both zero");
        }
    }
}
async fn act2(state: Arc<Mutex<State>>){
    while state.lock().await.c2 > 0 {
        tokio::time::sleep(Duration::from_secs(1)).await;
        state.lock().await.c2 -=1;
        if state.lock().await.c1 == 0 && state.lock().await.c2 == 0 {
            println!("both zero");
        }
    }
}
#[tokio::main]
async fn main(){
    let state = State {
        c1: 10,
        c2: 5
    };
    let state = Arc::new(Mutex::new(state));
    let state2 = state.clone();
    let t1 = tokio::spawn( async {
        act1(state).await
    });
    let t2 = tokio::spawn( async {
        act2(state2).await
    });
    join!(t1,t2);
}
