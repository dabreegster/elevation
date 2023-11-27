use std::io::BufReader;

use fs_err::File;
use georaster::geotiff::{GeoTiffReader, RasterValue};

fn main() {
    let seattle = "/home/dabreegster/abstreet/data/input/shared/elevation/kc_2016_lidar.tif";
    let uk = "/home/dabreegster/abstreet/data/input/shared/elevation/UK-dem-50m-4326.tif";

    let mut reader = Reader::new(BufReader::new(File::open(uk).unwrap()));

    // https://www.openstreetmap.org/node/719065360, Parliament hill, should be 93m
    let lon = -0.1597557;
    let lat = 51.5596469;
    println!("{:?}", reader.get_height_for_lon_lat(lon, lat));
}

struct Reader {
    tiff: GeoTiffReader<BufReader<File>>,
    origin: [f64; 2],
    pixel_size: [f64; 2],
}

impl Reader {
    fn new(file: BufReader<File>) -> Self {
        let tiff = GeoTiffReader::open(file).unwrap();
        let origin = tiff.origin().unwrap();
        let pixel_size = tiff.pixel_size().unwrap();
        Self {
            tiff,
            origin,
            pixel_size,
        }
    }

    // Assumed to be meters
    // Assumes the tiff is in EPSG:4326
    fn get_height_for_lon_lat(&mut self, lon: f64, lat: f64) -> Option<f32> {
        // Just linearly interpolate
        let x = ((lon - self.origin[0]) / self.pixel_size[0]) as u32;
        let y = ((lat - self.origin[1]) / self.pixel_size[1]) as u32;
        if let RasterValue::F32(height) = self.tiff.read_pixel(x, y) {
            return Some(height);
        }
        None
    }
}
