mod mandelbrot;

use mandelbrot::{calculate_mandelbrot, render_mandelbrot};

fn main() {
    render_mandelbrot(calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 100, 24))
}
