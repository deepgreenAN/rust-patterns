fn main() {
    use adapter::{LegacyCVAdapter, SurveillanceCamera};

    let surveillance_camera = SurveillanceCamera::<LegacyCVAdapter>::new();

    surveillance_camera.run_loop();
}
