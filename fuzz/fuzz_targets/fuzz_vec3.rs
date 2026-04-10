#![no_main]
use libfuzzer_sys::fuzz_target;
use oxidized_mc_types::Vec3;

fuzz_target!(|data: [f64; 6]| {
    let [x1, y1, z1, x2, y2, z2] = data;

    let a = Vec3::new(x1, y1, z1);
    let b = Vec3::new(x2, y2, z2);

    // Arithmetic must not panic.
    let _ = a + b;
    let _ = a - b;
    let _ = -a;
    let _ = a.length_squared();
    let _ = a.length();
    let _ = a.dot(b);
    let _ = a.cross(b);
    let _ = a.normalize();
    let _ = a.scale(x2);
    let _ = a.distance_to(b);
    let _ = a.distance_to_sqr(b);
});
