pub struct ColorScheme {
    pub colors: Vec<[u8; 3]>,
}

pub fn create_color_scheme() -> ColorScheme {
    ColorScheme {
        colors: vec![
            [66, 30, 15],
            [25, 7, 26],
            [9, 1, 47],
            [4, 4, 73],
            [0, 7, 100],
            [12, 44, 138],
            [24, 82, 177],
            [57, 125, 209],
            [134, 181, 229],
            [211, 236, 248],
            [241, 233, 191],
            [248, 201, 95],
            [255, 170, 0],
            [204, 128, 0],
            [153, 87, 0],
            [106, 52, 3],
        ],
    }
}

impl ColorScheme {
    pub fn map(&self, value: f64, max_iterations: u32) -> [u8; 3] {
        let index = (value * (self.colors.len() - 1) as f64 / max_iterations as f64).floor() as usize;
        self.colors[index]
    }
}
