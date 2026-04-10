#![allow(missing_docs)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use oxidized_mc_types::{
    Aabb, Axis, BlockPos, Direction, ResourceLocation, SectionPos, Vec3, Vec3i,
};

// ---------------------------------------------------------------------------
// BlockPos
// ---------------------------------------------------------------------------

fn bench_block_pos(c: &mut Criterion) {
    let mut g = c.benchmark_group("block_pos");

    let pos = BlockPos::new(18_357_644, 64, -20_882_616);

    g.bench_function("new", |b| {
        b.iter(|| BlockPos::new(black_box(18_357_644), black_box(64), black_box(-20_882_616)));
    });

    g.bench_function("as_long", |b| {
        b.iter(|| black_box(pos).as_long());
    });

    let packed = pos.as_long();
    g.bench_function("from_long", |b| {
        b.iter(|| BlockPos::from_long(black_box(packed)));
    });

    g.bench_function("as_long_roundtrip", |b| {
        b.iter(|| BlockPos::from_long(black_box(pos).as_long()));
    });

    g.bench_function("offset", |b| {
        b.iter(|| black_box(pos).offset(1, -1, 1));
    });

    g.bench_function("relative_direction", |b| {
        b.iter(|| black_box(pos).relative(black_box(Direction::North)));
    });

    g.bench_function("above", |b| {
        b.iter(|| black_box(pos).above());
    });

    g.bench_function("containing", |b| {
        b.iter(|| BlockPos::containing(black_box(1.5), black_box(64.7), black_box(-3.2)));
    });

    let other = BlockPos::new(18_357_700, 70, -20_882_500);
    g.bench_function("dist_sqr", |b| {
        b.iter(|| black_box(pos).dist_sqr(black_box(&other)));
    });

    g.bench_function("get_center", |b| {
        b.iter(|| black_box(pos).get_center());
    });

    g.finish();
}

// ---------------------------------------------------------------------------
// SectionPos
// ---------------------------------------------------------------------------

fn bench_section_pos(c: &mut Criterion) {
    let mut g = c.benchmark_group("section_pos");

    let pos = SectionPos::new(100, 4, -200);

    g.bench_function("as_long", |b| {
        b.iter(|| black_box(pos).as_long());
    });

    let packed = pos.as_long();
    g.bench_function("from_long", |b| {
        b.iter(|| SectionPos::from_long(black_box(packed)));
    });

    g.bench_function("as_long_roundtrip", |b| {
        b.iter(|| SectionPos::from_long(black_box(pos).as_long()));
    });

    let block = BlockPos::new(1600, 64, -3200);
    g.bench_function("of_block_pos", |b| {
        b.iter(|| SectionPos::of_block_pos(black_box(&block)));
    });

    g.bench_function("block_to_section_coord", |b| {
        b.iter(|| SectionPos::block_to_section_coord(black_box(1600)));
    });

    g.finish();
}

// ---------------------------------------------------------------------------
// Vec3
// ---------------------------------------------------------------------------

fn bench_vec3(c: &mut Criterion) {
    let mut g = c.benchmark_group("vec3");

    let a = Vec3::new(3.0, 4.0, 5.0);
    let b = Vec3::new(1.0, -2.0, 3.0);

    g.bench_function("normalize", |b_iter| {
        b_iter.iter(|| black_box(a).normalize());
    });

    g.bench_function("dot", |b_iter| {
        b_iter.iter(|| black_box(a).dot(black_box(b)));
    });

    g.bench_function("cross", |b_iter| {
        b_iter.iter(|| black_box(a).cross(black_box(b)));
    });

    g.bench_function("length_sqr", |b_iter| {
        b_iter.iter(|| black_box(a).length_sqr());
    });

    g.bench_function("add_vec", |b_iter| {
        b_iter.iter(|| black_box(a).add_vec(black_box(b)));
    });

    g.bench_function("add_operator", |b_iter| {
        b_iter.iter(|| black_box(a) + black_box(b));
    });

    g.bench_function("distance_to_sqr", |b_iter| {
        b_iter.iter(|| black_box(a).distance_to_sqr(black_box(b)));
    });

    g.bench_function("scale", |b_iter| {
        b_iter.iter(|| black_box(a).scale(black_box(2.5)));
    });

    g.bench_function("lerp", |b_iter| {
        b_iter.iter(|| black_box(a).lerp(black_box(b), black_box(0.5)));
    });

    g.finish();
}

// ---------------------------------------------------------------------------
// Aabb
// ---------------------------------------------------------------------------

fn bench_aabb(c: &mut Criterion) {
    let mut g = c.benchmark_group("aabb");

    let a = Aabb::from_vec3(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 1.0));
    let b = Aabb::from_vec3(Vec3::new(0.5, 0.5, 0.5), Vec3::new(1.5, 2.5, 1.5));
    let miss = Aabb::from_vec3(Vec3::new(10.0, 10.0, 10.0), Vec3::new(11.0, 11.0, 11.0));

    g.bench_function("intersects_hit", |bench| {
        bench.iter(|| black_box(a).intersects(black_box(&b)));
    });

    g.bench_function("intersects_miss", |bench| {
        bench.iter(|| black_box(a).intersects(black_box(&miss)));
    });

    g.bench_function("contains", |bench| {
        bench.iter(|| black_box(a).contains(black_box(0.5), black_box(1.0), black_box(0.5)));
    });

    g.bench_function("inflate", |bench| {
        bench.iter(|| black_box(a).inflate(black_box(0.5)));
    });

    g.bench_function("move_by", |bench| {
        bench.iter(|| black_box(a).move_by(black_box(1.0), black_box(2.0), black_box(3.0)));
    });

    g.bench_function("expand_towards", |bench| {
        bench.iter(|| black_box(a).expand_towards(black_box(0.5), black_box(-0.3), black_box(0.1)));
    });

    g.bench_function("from_vec3", |bench| {
        let p1 = Vec3::new(0.0, 0.0, 0.0);
        let p2 = Vec3::new(1.0, 2.0, 1.0);
        bench.iter(|| Aabb::from_vec3(black_box(p1), black_box(p2)));
    });

    g.bench_function("get_center", |bench| {
        bench.iter(|| black_box(a).get_center());
    });

    let point = Vec3::new(0.5, 1.0, 0.5);
    g.bench_function("distance_to_sqr", |bench| {
        bench.iter(|| black_box(miss).distance_to_sqr(black_box(point)));
    });

    g.finish();
}

// ---------------------------------------------------------------------------
// ResourceLocation (allocating — expected to be slower)
// ---------------------------------------------------------------------------

fn bench_resource_location(c: &mut Criterion) {
    let mut g = c.benchmark_group("resource_location");

    g.bench_function("parse_namespaced", |b| {
        b.iter(|| ResourceLocation::from_string(black_box("minecraft:diamond_sword")));
    });

    g.bench_function("parse_default_ns", |b| {
        b.iter(|| ResourceLocation::from_string(black_box("stone")));
    });

    g.bench_function("parse_long_path", |b| {
        b.iter(|| {
            ResourceLocation::from_string(black_box("minecraft:textures/block/acacia_planks.png"))
        });
    });

    g.finish();
}

// ---------------------------------------------------------------------------
// Direction
// ---------------------------------------------------------------------------

fn bench_direction(c: &mut Criterion) {
    let mut g = c.benchmark_group("direction");

    let dir = Direction::North;

    g.bench_function("step_x", |b| {
        b.iter(|| black_box(dir).step_x());
    });

    g.bench_function("step_y", |b| {
        b.iter(|| black_box(dir).step_y());
    });

    g.bench_function("step_z", |b| {
        b.iter(|| black_box(dir).step_z());
    });

    g.bench_function("opposite", |b| {
        b.iter(|| black_box(dir).opposite());
    });

    g.bench_function("axis", |b| {
        b.iter(|| black_box(dir).axis());
    });

    g.bench_function("get_nearest", |b| {
        b.iter(|| Direction::get_nearest(black_box(0.3), black_box(-0.8), black_box(0.1)));
    });

    g.finish();
}

// ---------------------------------------------------------------------------
// Vec3i
// ---------------------------------------------------------------------------

fn bench_vec3i(c: &mut Criterion) {
    let mut g = c.benchmark_group("vec3i");

    let a = Vec3i::new(100, 64, -200);
    let b = Vec3i::new(50, 32, -100);

    g.bench_function("offset", |bench| {
        bench.iter(|| black_box(a).offset(black_box(1), black_box(-1), black_box(1)));
    });

    g.bench_function("cross", |bench| {
        bench.iter(|| black_box(a).cross(black_box(b)));
    });

    g.bench_function("dist_sqr", |bench| {
        bench.iter(|| black_box(a).dist_sqr(black_box(b)));
    });

    g.bench_function("relative", |bench| {
        bench.iter(|| black_box(a).relative(black_box(Direction::East)));
    });

    g.bench_function("add_operator", |bench| {
        bench.iter(|| black_box(a) + black_box(b));
    });

    g.bench_function("get_axis", |bench| {
        bench.iter(|| black_box(a).get(black_box(Axis::Y)));
    });

    g.finish();
}

// ---------------------------------------------------------------------------
// Registration
// ---------------------------------------------------------------------------

criterion_group!(
    benches,
    bench_block_pos,
    bench_section_pos,
    bench_vec3,
    bench_aabb,
    bench_resource_location,
    bench_direction,
    bench_vec3i,
);

criterion_main!(benches);
