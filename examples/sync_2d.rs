#[cfg(not(any(target_arch = "wasm32", target_os = "ios")))]
#[kiss3d::main]
async fn main() {
    use kiss3d::prelude::*;

    let mut window = Window::new("Kiss3d: sync 2D").await;
    window.set_vsync(false);
    let mut camera = PanZoomCamera2d::new(Vec2::ZERO, 1.0);
    let mut scene = SceneNode2d::empty();
    let mut rectangle = scene.add_rectangle(180.0, 120.0).set_color(RED);

    while window.sync_render_2d(&mut scene, &mut camera) {
        rectangle.rotate(0.014);
    }
}

#[cfg(any(target_arch = "wasm32", target_os = "ios"))]
fn main() {
    panic!("This example uses a desktop loop; use a platform frame callback on web/iOS.");
}
