use std::{thread, time};

fn add (a: i32, b: i32) -> i32{
    a+b
}

pub fn two_threads(){
    let start = time::Instant::now();

    let handler_1 = thread::spawn( || {
        let pause = time::Duration::from_millis(500);
        thread::sleep(pause.clone());
    });

    let handler_2 = thread::spawn(||{
        let pause = time::Duration::from_millis(300);
        thread::sleep(pause.clone());
    });

    handler_1.join().unwrap();
    handler_2.join().unwrap();

    let finish = time::Instant::now();

    println!{"{:02?}", finish.duration_since(start)};
}

pub fn many_threads_sleep(){
    for n in 1..=1000{
        let mut handlers:Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);

        let start = time::Instant::now();
        for _m in 0..n{
            let handle = thread::spawn(||{
                let pause = time::Duration::from_millis(20);
                thread::sleep(pause);
            });
            handlers.push(handle);
        }

        while let Some(handle) = handlers.pop(){
            handle.join();
        }

        let finish = time::Instant::now();
        println!{"{}\t{:02?}", n, finish.duration_since(start)};
    }
}

pub fn many_threads_busyweight(){
    for n in 1..=1000{
        let mut handlers:Vec<thread::JoinHandle<()>> = Vec::with_capacity(n);

        let start = time::Instant::now();
        for _m in 0..n{
            let handle = thread::spawn(||{
                let start = time::Instant::now();
                let pause = time::Duration::from_millis(20);
                while start.elapsed() < pause{
                    thread::yield_now();
                }
            });
            handlers.push(handle);
        }

        while let Some(handle) = handlers.pop(){
            handle.join();
        }

        let finish = time::Instant::now();
        println!{"{}\t{:02?}", n, finish.duration_since(start)};
    }
}

use crossbeam::channel::unbounded;
pub fn channel_intro(){
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        tx.send(109).unwrap();  
    });

    select!{
        recv(rx) -> msg => println!("{:?}", msg),
    }
}

#[derive(Debug)]
enum ConnectivityCheck{
    Ping,
    Pong,
    Pang,
}

pub fn channel_duplex(){
    let n_message = 3;
    let (requestes_tx, requests_rx) = unbounded();
    let (responses_tx, responses_rx) = unbounded();
    
    thread::spawn(move || loop{
        match requests_rx.recv().unwrap(){
            ConnectivityCheck::Pong => eprintln!("unexpected pong response"),
            ConnectivityCheck::Ping => responses_tx.send(ConnectivityCheck::Pong).unwrap(),
            ConnectivityCheck::Pang => return,
        }
    });

    for _ in 0..n_message{
        requestes_tx.send(ConnectivityCheck::Ping).unwrap();
    }
    requestes_tx.send(ConnectivityCheck::Pang).unwrap();

    for _ in 0..n_message{
        select!{
            recv(responses_rx) -> msg => println!("{:?}", msg),
        }
    }
}