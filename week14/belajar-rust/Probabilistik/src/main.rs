use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut position = (0, 0);
    let goal = (10, 10);

    while position != goal {
        let uncertainty_x: f64 = rng.gen_range(-0.5..0.5);
        let uncertainty_y: f64 = rng.gen_range(-0.5..0.5);

        let step_x = if position.0 < goal.0 { 1 } else { 0 };
        let step_y = if position.1 < goal.1 { 1 } else { 0 };

        position.0 = ((position.0 as f64 + step_x as f64 + uncertainty_x).round() as i32).max(0) as usize;
        position.1 = ((position.1 as f64 + step_y as f64 + uncertainty_y).round() as i32).max(0) as usize;

        println!("Posisi saat ini: ({}, {})", position.0, position.1);
    }

    println!("Robot mencapai tujuan di ({}, {}).", goal.0, goal.1);
}
