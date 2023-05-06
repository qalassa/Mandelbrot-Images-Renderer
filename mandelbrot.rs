use crate::color::ColorScheme;
use crate::pic::Image;
use crate::adaptiveSampling::AdaptiveSampler;
use crate::gpuAcceleration::GpuAccelerator;

pub struct Mandelbrot {
    config: MandelbrotConfig,
}

pub struct MandelbrotConfig {
    pub width: u32,
    pub height: u32,
    pub max_iterations: u32,
    pub color_scheme: ColorScheme,
    pub adaptive_sampler: AdaptiveSampler,
    pub gpu_accelerator: GpuAccelerator,
    pub center: (f64, f64),
    pub zoom: f64,
}

impl Mandelbrot {
    pub fn new(config: MandelbrotConfig) -> Self {
        Mandelbrot { config }
    }

    pub fn render(&self, frame: u32) -> Image {
        let mut image = Image::new(self.config.width, self.config.height);

        self.config.gpu_accelerator.prepare();

        let scale = self.config.zoom * 2.0f64.powf(-(frame as f64) / 30.0);
        let (center_x, center_y) = self.config.center;

        for y in 0..self.config.height {
            for x in 0..self.config.width {
                let cx = (x as f64 / self.config.width as f64 - 0.5) * scale + center_x;
                let cy = (y as f64 / self.config.height as f64 - 0.5) * scale + center_y;

                let sample = self.config.adaptive_sampler.sample(cx, cy);
                let color = self.config.color_scheme.map(sample, self.config.max_iterations);

                image.set_pixel(x, y, color);
            }
        }

        image
    }
}

