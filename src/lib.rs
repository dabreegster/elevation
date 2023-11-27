use std::io::{Read, Seek};

use georaster::geotiff::{GeoTiffReader, RasterValue};

/// Reads elevation data from GeoTiff files
pub struct GeoTiffElevation<R: Read + Seek + Send> {
    tiff: GeoTiffReader<R>,
    top_left: [f64; 2],
    pixel_size: [f64; 2],
}

impl<R: Read + Seek + Send> GeoTiffElevation<R> {
    /// Pass in a geotiff file in EPSG:4326
    pub fn new(file: R) -> Self {
        let tiff = GeoTiffReader::open(file).unwrap();
        let top_left = tiff.origin().unwrap();
        let pixel_size = tiff.pixel_size().unwrap();
        Self {
            tiff,
            top_left,
            pixel_size,
        }
    }

    /// The height unit depends on the input file; usually meters
    pub fn get_height_for_lon_lat(&mut self, lon: f64, lat: f64) -> Option<f32> {
        // Just linearly interpolate
        let x = ((lon - self.top_left[0]) / self.pixel_size[0]) as u32;
        let y = ((lat - self.top_left[1]) / self.pixel_size[1]) as u32;
        if let RasterValue::F32(height) = self.tiff.read_pixel(x, y) {
            return Some(height);
        }
        None
    }

    /// Returns (x1, y1, x2, y2) where x is longitude and y is latitutde
    pub fn get_bounds(&self) -> (f64, f64, f64, f64) {
        let (width, height) = self.tiff.images()[0].dimensions.unwrap();
        let bottom_right_lon = self.top_left[0] + self.pixel_size[0] * (width as f64);
        let bottom_right_lat = self.top_left[1] + self.pixel_size[1] * (height as f64);

        // TODO Might be flipped, the names are not right
        let x1 = self.top_left[0].min(bottom_right_lon);
        let x2 = self.top_left[0].max(bottom_right_lon);
        let y1 = self.top_left[1].min(bottom_right_lat);
        let y2 = self.top_left[1].max(bottom_right_lat);

        (x1, y1, x2, y2)
    }
}
