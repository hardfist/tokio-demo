use std::time::Duration;

use tokio::sync::mpsc;

enum Event {
    Dec1,
    Dec2,
}
struct State {
    c1: u32,
    c2: u32,
}
#[tokio::main]
async fn main() {
    let mut state = State { c1: 10, c2: 5 };

    let (send, mut recv) = mpsc::channel::<Event>(8);
    let (s1, mut r1) = mpsc::channel(8);
    let (s2, mut r2) = mpsc::channel(8);
    let send1 = send.clone();
    let send2 = send.clone();
    tokio::spawn(async move {
        while let Some(msg) = r1.recv().await {
            println!("decl1");
            tokio::time::sleep(Duration::from_secs(1)).await;
            send1.send(Event::Dec1).await.unwrap()
        }
    });
    tokio::spawn(async move {
         while let Some(msg) = r2.recv().await {
            println!("decl2");
            tokio::time::sleep(Duration::from_secs(1)).await;
            send2.send(Event::Dec2).await.unwrap();
        }
    });
    send.send(Event::Dec1).await.unwrap();
    send.send(Event::Dec2).await.unwrap();
    while let Some(event) = recv.recv().await {
        match event {
            Event::Dec1 => {
                state.c1 -= 1;
                if state.c1 > 0 {
                    s1.send(()).await.unwrap();
                }
            }
            Event::Dec2 => {
                state.c2 -= 1;
                if state.c2 > 0 {
                    
                    s2.send(()).await.unwrap();
                }
            }
        }
        if state.c1 == 0 && state.c2 == 0 {
            println!("both are zero");
            break;
        }
    }
    drop(s1);
    drop(s2);
}
