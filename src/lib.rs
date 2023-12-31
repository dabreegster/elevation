use std::io::{Read, Seek};

use georaster::geotiff::{GeoTiffReader, RasterValue};

/// Reads elevation data from GeoTiff files
pub struct GeoTiffElevation<R: Read + Seek + Send> {
    tiff: GeoTiffReader<R>,
    top_left: [f32; 2],
    pixel_size: [f32; 2],
}

impl<R: Read + Seek + Send> GeoTiffElevation<R> {
    /// Pass in a geotiff file in EPSG:4326
    pub fn new(file: R) -> Self {
        let tiff = GeoTiffReader::open(file).unwrap();
        let top_left = tiff.origin().unwrap();
        let pixel_size = tiff.pixel_size().unwrap();
        Self {
            tiff,
            top_left: [top_left[0] as f32, top_left[1] as f32],
            pixel_size: [pixel_size[0] as f32, pixel_size[1] as f32],
        }
    }

    /// Uses bilinear interpolation. The height unit depends on the input file; usually meters.
    pub fn get_height_for_lon_lat(&mut self, lon: f32, lat: f32) -> Option<f32> {
        let x = (lon - self.top_left[0]) / self.pixel_size[0];
        let y = (lat - self.top_left[1]) / self.pixel_size[1];

        let x1 = x.floor();
        let x2 = x.ceil();
        let y1 = y.floor();
        let y2 = y.ceil();

        // Repeated linear interpolation formula from
        // https://en.wikipedia.org/wiki/Bilinear_interpolation
        let fraction = 1.0 / ((x2 - x1) * (y2 - y1));
        let term1 = fraction * self.get_value(x1, y1)? * (x2 - x) * (y2 - y);
        let term2 = self.get_value(x2, y1)? * (x - x1) * (y2 - y);
        let term3 = self.get_value(x1, y2)? * (x2 - x) * (y - y1);
        let term4 = self.get_value(x2, y2)? * (x - x1) * (y - y1);

        Some(term1 + term2 + term3 + term4)
    }

    /// Like `get_height_for_lon_lat`, but doesn't do any interpolation. The results won't be quite
    /// right.
    pub fn get_height_for_lon_lat_fast(&mut self, lon: f32, lat: f32) -> Option<f32> {
        let x = (lon - self.top_left[0]) / self.pixel_size[0];
        let y = (lat - self.top_left[1]) / self.pixel_size[1];
        self.get_value(x, y)
    }

    fn get_value(&mut self, x: f32, y: f32) -> Option<f32> {
        if let RasterValue::F32(height) = self.tiff.read_pixel(x as u32, y as u32) {
            Some(height)
        } else {
            None
        }
    }

    /// Returns (x1, y1, x2, y2) where x is longitude and y is latitutde
    pub fn get_bounds(&self) -> (f32, f32, f32, f32) {
        let (width, height) = self.tiff.images()[0].dimensions.unwrap();
        let bottom_right_lon = self.top_left[0] + self.pixel_size[0] * (width as f32);
        let bottom_right_lat = self.top_left[1] + self.pixel_size[1] * (height as f32);

        // TODO Might be flipped, the names are not right
        let x1 = self.top_left[0].min(bottom_right_lon);
        let x2 = self.top_left[0].max(bottom_right_lon);
        let y1 = self.top_left[1].min(bottom_right_lat);
        let y2 = self.top_left[1].max(bottom_right_lat);

        (x1, y1, x2, y2)
    }
}

#[cfg(test)]
mod tests {
    use super::GeoTiffElevation;
    use rand::Rng;

    // TODO Use criterion?
    #[test]
    fn benchmark() {
        let seattle = "/home/dabreegster/abstreet/data/input/shared/elevation/seattle.tif";
        let uk = "/home/dabreegster/abstreet/data/input/shared/elevation/UK-dem-50m-4326.tif";
        let mut elevation =
            GeoTiffElevation::new(std::io::BufReader::new(std::fs::File::open(uk).unwrap()));

        let iterations = 500_000;

        let (x1, y1, x2, y2) = elevation.get_bounds();
        println!("Input ranges from {x1}, {y1} to {x2}, {y2}");
        println!("Running {iterations} iterations");
        let mut rng = rand::thread_rng();
        for _ in 0..iterations {
            let lon = rng.gen_range(x1..x2);
            let lat = rng.gen_range(y1..y2);
            if let Some(height) = elevation.get_height_for_lon_lat(lon, lat) {
                if !height.is_finite() {
                    panic!("lon {lon}, lat {lat} yielded invalid {height}");
                }
            }
        }
    }

    #[test]
    fn exact_point() {
        let uk = "/home/dabreegster/abstreet/data/input/shared/elevation/UK-dem-50m-4326.tif";
        let mut elevation =
            GeoTiffElevation::new(std::io::BufReader::new(std::fs::File::open(uk).unwrap()));

        // Make sure the bilinear interpolation doesn't break for input directly on a corner
        for (dx, dy) in [(0.0, 0.0), (0.1, 0.0), (0.1, 0.1), (0.0, 0.1)] {
            // Manually found some point that isn't at 0 meters
            let x = elevation.top_left[0] + (11382.0 + dx) * elevation.pixel_size[0];
            let y = elevation.top_left[1] + (7550.0 + dy) * elevation.pixel_size[1];

            let height = elevation.get_height_for_lon_lat(x, y).unwrap();
            if !height.is_finite() {
                panic!("for dx {dx} and dy {dy}, got {height}");
            }
        }
    }
}
