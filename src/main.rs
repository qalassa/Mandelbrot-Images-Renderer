mod mandelbrot;
mod pic;
mod color;
mod adaptiveSampling;
mod gpuAcceleration;

use mandelbrot::{Mandelbrot, MandelbrotConfig};
use pic::Image;
use color::create_color_scheme;
use adaptiveSampling::AdaptiveSampler;
use gpuAcceleration::GpuAccelerator;

fn main() {
    let config = MandelbrotConfig {
        width: 1920,
        height: 1080,
        max_iterations: 1000,
        color_scheme: create_color_scheme(),
        adaptive_sampler: AdaptiveSampler::new(),
        gpu_accelerator: GpuAccelerator::new(),
        center: (-0.5, 0.0),
        zoom: 3.0,
    };

    let mandelbrot = Mandelbrot::new(config);
    let total_frames = 180; // 6 seconds at 30 FPS

    for frame in 0..total_frames {
        let image = mandelbrot.render(frame);
        image.save(format!("mandelbrot_{:04}.png", frame).as_str()).unwrap();
    }

    println!("All frames rendered.");
}
