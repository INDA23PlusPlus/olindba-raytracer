use raytracer::util::{old_vector::OldVec3, vector::Vec3, random::Random};

fn main() {
    benchmarking::warm_up();
    cross();
    normalize();
    reflect();
    scalar();
    project();
}

fn cross() {
    let mut rng = Random::new(26754825);

    let bench_result1 = benchmarking::measure_function(|measurer| {
        let vec1 = Vec3::new([rng.next(), rng.next(), rng.next()]);
        let vec2 = Vec3::new([rng.next(), rng.next(), rng.next()]);
        measurer.measure(|| {
            for _ in 0..1000000 {
                vec1.cross(vec2);
            }
        });
    });

    let bench_result2 = benchmarking::measure_function(|measurer| {
        let vec1 = OldVec3::new([rng.next_f64(), rng.next_f64(), rng.next_f64()]);
        let vec2 = OldVec3::new([rng.next_f64(), rng.next_f64(), rng.next_f64()]);
        measurer.measure(|| {
            for _ in 0..1000000 {
                vec1.cross(vec2);
            }
        });
    });

    println!("Cross product: SISD {:?}, SIMD {:?}", bench_result2.unwrap().elapsed(), bench_result1.unwrap().elapsed());
}

fn normalize() {
    let mut rng = Random::new(26754825);

    let bench_result1 = benchmarking::measure_function(|measurer| {
        let vec1 = Vec3::new([rng.next(), rng.next(), rng.next()]);
        measurer.measure(|| {
            for _ in 0..1000000 {
                vec1.normalized();
            }
        });
    });

    let bench_result2 = benchmarking::measure_function(|measurer| {
        let vec1 = OldVec3::new([rng.next_f64(), rng.next_f64(), rng.next_f64()]);
        measurer.measure(|| {
            for _ in 0..1000000 {
                vec1.normalized();
            }
        });
    });

    println!("Normalize: SISD {:?}, SIMD {:?}", bench_result2.unwrap().elapsed(), bench_result1.unwrap().elapsed());
}

fn reflect() {
    let mut rng = Random::new(26754825);

    let bench_result1 = benchmarking::measure_function(|measurer| {
        let vec1 = Vec3::new([rng.next(), rng.next(), rng.next()]);
        let vec2 = Vec3::new([rng.next(), rng.next(), rng.next()]);
        measurer.measure(|| {
            for _ in 0..1000000 {
                vec1.reflect(vec2);
            }
        });
    });

    let bench_result2 = benchmarking::measure_function(|measurer| {
        let vec1 = OldVec3::new([rng.next_f64(), rng.next_f64(), rng.next_f64()]);
        let vec2 = OldVec3::new([rng.next_f64(), rng.next_f64(), rng.next_f64()]);
        measurer.measure(|| {
            for _ in 0..1000000 {
                vec1.reflect(vec2);
            }
        });
    });

    println!("Reflect vector: SISD {:?}, SIMD {:?}", bench_result2.unwrap().elapsed(), bench_result1.unwrap().elapsed());
}

fn scalar() {
    let mut rng = Random::new(26754825);

    let bench_result1 = benchmarking::measure_function(|measurer| {
        let vec1 = Vec3::new([rng.next(), rng.next(), rng.next()]);
        measurer.measure(|| {
            for _ in 0..1000000 {
                let _ = vec1 * rng.next();
            }
        });
    });

    let bench_result2 = benchmarking::measure_function(|measurer| {
        let vec1 = OldVec3::new([rng.next_f64(), rng.next_f64(), rng.next_f64()]);
        measurer.measure(|| {
            for _ in 0..1000000 {
                let _ = vec1 * rng.next_f64();
            }
        });
    });

    println!("Scalar product: SISD {:?}, SIMD {:?}", bench_result2.unwrap().elapsed(), bench_result1.unwrap().elapsed());
}

fn project() {
    let mut rng = Random::new(26754825);

    let bench_result1 = benchmarking::measure_function(|measurer| {
        let vec1 = Vec3::new([rng.next(), rng.next(), rng.next()]);
        let vec2 = Vec3::new([rng.next(), rng.next(), rng.next()]);
        measurer.measure(|| {
            for _ in 0..1000000 {
                vec1.project(vec2);
            }
        });
    });

    let bench_result2 = benchmarking::measure_function(|measurer| {
        let vec1 = OldVec3::new([rng.next_f64(), rng.next_f64(), rng.next_f64()]);
        let vec2 = OldVec3::new([rng.next_f64(), rng.next_f64(), rng.next_f64()]);
        measurer.measure(|| {
            for _ in 0..1000000 {
                vec1.project(vec2);
            }
        });
    });

    println!("Dot product: SISD {:?}, SIMD {:?}", bench_result2.unwrap().elapsed(), bench_result1.unwrap().elapsed());
}
