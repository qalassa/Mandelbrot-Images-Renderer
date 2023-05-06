pub struct AdaptiveSampler;

impl AdaptiveSampler {
    pub fn new() -> Self {
        AdaptiveSampler
    }

    pub fn sample(&self, x: f64, y: f64) -> f64 {
        let c = num_complex::Complex::new(x, y);
        let mut z = c;

        for i in 0..1000 {
            if z.norm() > 2.0 {
                return i as f64;
            }
            z = z * z + c;
        }

        0.0
    }
}

