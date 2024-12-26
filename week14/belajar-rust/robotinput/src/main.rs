use std::io;

fn main() {
    let mut position = (0, 0); // Posisi awal robot

    loop {
        println!("Posisi saat ini: ({}, {})", position.0, position.1);
        println!("Masukkan gerakan (w/a/s/d untuk atas/kiri/bawah/kanan, q untuk keluar):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Gagal membaca input");

        match input.trim() {
            "w" => position.1 += 1,
            "a" => position.0 -= 1,
            "s" => position.1 -= 1,
            "d" => position.0 += 1,
            "q" => {
                println!("Keluar program. Posisi terakhir: ({}, {})", position.0, position.1);
                break;
            }
            _ => println!("Input tidak valid! Gunakan w/a/s/d/q."),
        }
    }
}
