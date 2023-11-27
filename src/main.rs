use std::io::BufReader;

use fs_err::File;
use rand::Rng;

use elevation::GeoTiffElevation;

fn main() {
    let seattle = "/home/dabreegster/abstreet/data/input/shared/elevation/seattle.tif";
    let uk = "/home/dabreegster/abstreet/data/input/shared/elevation/UK-dem-50m-4326.tif";

    let mut elevation = GeoTiffElevation::new(BufReader::new(File::open(seattle).unwrap()));

    if false {
        // https://www.openstreetmap.org/node/719065360, Parliament hill, should be 93m
        let lon = -0.1597557;
        let lat = 51.5596469;
        println!("{:?}", elevation.get_height_for_lon_lat(lon, lat));
    }

    if true {
        benchmark(&mut elevation, 500_000);
    }
}

fn benchmark(elevation: &mut GeoTiffElevation<BufReader<File>>, iterations: usize) {
    let (x1, y1, x2, y2) = elevation.get_bounds();
    println!("Input ranges from {x1}, {y1} to {x2}, {y2}");
    println!("Running {iterations} iterations");
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        let lon = rng.gen_range(x1..x2);
        let lat = rng.gen_range(y1..y2);
        elevation.get_height_for_lon_lat(lon, lat);
    }
}
