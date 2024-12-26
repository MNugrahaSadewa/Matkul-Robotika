use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Task {
    priority: u32,
    description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority) // Higher priority comes first
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut task_queue = BinaryHeap::new();

    task_queue.push(Task {
        priority: 5,
        description: String::from("Mengisi baterai"),
    });
    task_queue.push(Task {
        priority: 10,
        description: String::from("Mengambil barang di lokasi A"),
    });
    task_queue.push(Task {
        priority: 1,
        description: String::from("Melakukan kalibrasi sensor"),
    });

    while let Some(task) = task_queue.pop() {
        println!("Menyelesaikan tugas: {} (Prioritas: {})", task.description, task.priority);
    }
}
