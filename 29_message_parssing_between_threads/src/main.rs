use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let msgs = vec![
            String::from("[T1] - Hi"),
            String::from("[T1] - From"),
            String::from("[T1] - The"),
            String::from("[T1] - Thread"),
        ];

        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let msgs = vec![
            String::from("[T2] - More"),
            String::from("[T2] - Messages"),
            String::from("[T2] - For"),
            String::from("[T2] - You"),
        ];

        for msg in msgs {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
