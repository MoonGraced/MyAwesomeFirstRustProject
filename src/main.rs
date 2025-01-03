fn main() {
    let start = -3.0f64;
    let end = 3.0f64;
    const NUM: usize = 123;
    let step = (end - start) / NUM as f64;
    let mut parts = [create_part(0.0f64, 0.0f64); NUM];
    for i in 0..NUM {
        parts[i] = create_part(start + i as f64 * step, start + (i as f64 + 1.0f64) * step);
        let val: f64 =  calculate_value(&mut parts[i]);
        println!("x_min {}, x_max {}, value {}", parts[i].x_min, parts[i].x_max, val)
    }
    for i in 0..NUM {
        println!("{}", f64_to_str(parts[i].value))
    }
}

#[derive(Copy, Clone)]
struct Part {
    x_min: f64,
    x_max: f64,
    value: f64,
}

fn normal_distribution(mu: f64, sigma: f64, x: f64) -> f64 {
    let f: f64= 1.0f64 / (sigma * (2.0f64 * std::f64::consts::PI).sqrt()) * (-0.5f64 * ((x - mu) / sigma).powi(2)).exp();
    f
}

fn create_part(x_min_val: f64, x_max_val: f64) -> Part {
    Part {
        x_min: x_min_val,
        x_max: x_max_val,
        value: 0.0f64
    }
}

fn calculate_value(part_ref: &mut Part) -> f64 {
    part_ref.value = normal_distribution(0.0f64, 1.0f64, part_ref.x_min + (part_ref.x_max - part_ref.x_min) / 2.0f64);
    part_ref.value
}

fn f64_to_str(number: f64) -> String {
    let num: u128 = (number * 500.0f64) as u128;
    let mut res: String = String::new();
    for _ in 0..num {
        res.push_str("x");
    }
    res
}