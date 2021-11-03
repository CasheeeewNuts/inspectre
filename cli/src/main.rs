use std::thread;
use std::sync::mpsc;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        sleep(Duration::from_secs(10));

        tx.send("thread end!").unwrap()
    });

    loop {
        let msg = rx.try_recv();

        if let Ok(msg) = msg {
            println!("{}", msg);
            break
        } else {
            println!("thread living")
        }

        sleep(Duration::from_millis(1000));
    }
}
