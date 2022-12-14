mod color;
mod ray;
mod renderer;

use renderer::Renderer;

fn main() {
    let renderer = Renderer::new(256, 256);

    renderer.render().unwrap();
}
