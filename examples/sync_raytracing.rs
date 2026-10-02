#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
#[kiss3d::main]
async fn main() {
    use kiss3d::prelude::*;
    use kiss3d::renderer::RayTracer;

    let mut window = Window::new("Kiss3d: sync ray tracing").await;
    window.set_vsync(false);
    let mut camera = OrbitCamera3d::new(Vec3::new(3.0, 2.5, 5.0), Vec3::ZERO);
    let mut scene = SceneNode3d::empty();
    scene.add_sphere(0.5).set_color(RED);
    scene
        .add_cube(8.0, 0.1, 8.0)
        .set_position(Vec3::new(0.0, -0.55, 0.0))
        .set_color(WHITE);
    scene
        .add_light(Light::point(100.0))
        .set_position(Vec3::new(-2.0, 3.0, 2.0));

    // Keep the scene still and reuse the tracer so samples can accumulate.
    let mut raytracer = RayTracer::new();
    raytracer.set_max_bounces(8);
    raytracer.set_denoise(true);
    raytracer.set_denoise_iterations(5);

    let font = Font::default();
    match raytracer.backend() {
        RayBackend::Software => println!("Path tracer backend: compute (BVH)"),
        RayBackend::Hardware => println!("Path tracer backend: hardware ray queries"),
    }

    while window.sync_raytrace_3d(&mut scene, &mut camera, &mut raytracer) {
        window.draw_text(
            &format!("Samples: {}", raytracer.samples_accumulated()),
            Vec2::new(10.0, 10.0),
            40.0,
            &font,
            WHITE,
        );
    }
}

#[cfg(any(target_arch = "wasm32", target_os = "ios"))]
fn main() {
    panic!("This example uses a desktop loop; use a platform frame callback on web/iOS.");
}
