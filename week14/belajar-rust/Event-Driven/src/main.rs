use std::sync::mpsc;
use std::thread;
use std::time::Duration;

enum Event {
    ObstacleDetected,
    GoalChanged(i32, i32),
}

fn main() {
    let (tx, rx) = mpsc::channel();

    // Simulasi sensor penghalang
    let sensor_tx = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        sensor_tx.send(Event::ObstacleDetected).unwrap();
    });

    // Simulasi perubahan tujuan
    let goal_tx = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(4));
        goal_tx.send(Event::GoalChanged(10, 15)).unwrap();
    });

    let mut position = (0, 0);

    loop {
        match rx.recv() {
            Ok(Event::ObstacleDetected) => {
                println!("Penghalang terdeteksi! Robot berhenti untuk menghindar.");
            }
            Ok(Event::GoalChanged(x, y)) => {
                println!("Tujuan baru diterima: ({}, {}). Robot bergerak ke tujuan.", x, y);
                position = (x, y);
            }
            Err(_) => break,
        }

        println!("Posisi robot saat ini: ({}, {})", position.0, position.1);
    }
}
