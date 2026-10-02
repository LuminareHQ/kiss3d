#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
#[kiss3d::main]
async fn main() {
    use kiss3d::prelude::*;

    let mut window = Window::new("Kiss3d: sync 3D").await;
    window.set_vsync(false);
    let mut camera = OrbitCamera3d::default();
    let mut scene = SceneNode3d::empty();
    scene
        .add_light(Light::point(100.0))
        .set_position(Vec3::new(0.0, 2.0, -2.0));
    let mut cube = scene.add_cube(1.0, 1.0, 1.0).set_color(RED);

    while window.sync_render_3d(&mut scene, &mut camera) {
        cube.rotate(Quat::from_rotation_y(0.014));
    }
}

#[cfg(any(target_arch = "wasm32", target_os = "ios"))]
fn main() {
    panic!("This example uses a desktop loop; use a platform frame callback on web/iOS.");
}
