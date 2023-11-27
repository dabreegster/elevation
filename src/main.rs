use std::io::BufReader;

use fs_err::File;
use georaster::geotiff::{GeoTiffReader, RasterValue};
use rand::Rng;

fn main() {
    let seattle = "/home/dabreegster/abstreet/data/input/shared/elevation/kc_2016_lidar.tif";
    let uk = "/home/dabreegster/abstreet/data/input/shared/elevation/UK-dem-50m-4326.tif";

    let mut reader = Reader::new(BufReader::new(File::open(uk).unwrap()));

    if false {
        // https://www.openstreetmap.org/node/719065360, Parliament hill, should be 93m
        let lon = -0.1597557;
        let lat = 51.5596469;
        println!("{:?}", reader.get_height_for_lon_lat(lon, lat));
    }

    if true {
        benchmark(&mut reader, 500_000);
    }
}

fn benchmark(reader: &mut Reader, iterations: usize) {
    // TODO Might be flipped, the names are not right
    let bottom_right = reader.get_bottom_right_lon_lat();
    let x1 = reader.top_left[0].min(bottom_right[0]);
    let x2 = reader.top_left[0].max(bottom_right[0]);
    let y1 = reader.top_left[1].min(bottom_right[1]);
    let y2 = reader.top_left[1].max(bottom_right[1]);

    println!("Running {iterations} iterations");
    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        let lon = rng.gen_range(x1..x2);
        let lat = rng.gen_range(y1..y2);
        reader.get_height_for_lon_lat(lon, lat);
    }
}

struct Reader {
    tiff: GeoTiffReader<BufReader<File>>,
    top_left: [f64; 2],
    pixel_size: [f64; 2],
}

impl Reader {
    fn new(file: BufReader<File>) -> Self {
        let tiff = GeoTiffReader::open(file).unwrap();
        let top_left = tiff.origin().unwrap();
        let pixel_size = tiff.pixel_size().unwrap();
        Self {
            tiff,
            top_left,
            pixel_size,
        }
    }

    // Assumed to be meters
    // Assumes the tiff is in EPSG:4326
    fn get_height_for_lon_lat(&mut self, lon: f64, lat: f64) -> Option<f32> {
        // Just linearly interpolate
        let x = ((lon - self.top_left[0]) / self.pixel_size[0]) as u32;
        let y = ((lat - self.top_left[1]) / self.pixel_size[1]) as u32;
        if let RasterValue::F32(height) = self.tiff.read_pixel(x, y) {
            return Some(height);
        }
        None
    }

    fn get_bottom_right_lon_lat(&self) -> [f64; 2] {
        let (width, height) = self.tiff.images()[0].dimensions.unwrap();
        let lon = self.top_left[0] + self.pixel_size[0] * (width as f64);
        let lat = self.top_left[1] + self.pixel_size[1] * (height as f64);
        [lon, lat]
    }
}
