use rand::*;

pub fn calculate() -> f32 {
    let n = 100000000;
    let mut m = 0;
    for _i in 0..=n {
        let px = random::<f32>() * 2.0 - 1.0;
        let py = random::<f32>() * 2.0 - 1.0;
        if px * px + py * py <= 1.0 {
            m += 1;
        }
    }
    4.0 * m as f32 / n as f32
}
