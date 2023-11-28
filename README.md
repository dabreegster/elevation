# elevation

Use [georaster](https://github.com/pka/georaster/) to read elevation data from GeoTIFFs in pure Rust.

This is really early work, but can already be used [in A/B
Street](https://github.com/a-b-street/abstreet/issues/82) to calculate the
height at intersections. This approach possibly replaces
[elevation_lookups](https://github.com/eldang/elevation_lookups), a much more
flexible Python and GDAL-based solution. Those dependencies are quite
heavyweight and have portability problems, even with Docker.

The approach in this repo is to stay as simple as possible, and only handle
GeoTIFF files in EPSG:4326, so that no dependencies are needed for coordinate
projection. In practice for A/B Street, once a DEM has been located, we have to
do a bit of manual work to use it anyway, so those steps can very much include
a one-time coordinate transformation.

## Data sources

Right now I've used this for only two sources.

- King County 2016 LIDAR, from <ftp://ftp.coast.noaa.gov/pub/DigitalCoast/raster2/elevation/WA_King_DEM_2016_8589/kingcounty_delivery1_be.tif>, in ESRI:103565.
  - I've converted by doing `gdalwarp -t_srs EPSG:4326 input.tif output.tif`, but there's room for improvement
- Ordnance Survey Terrain 50 for the UK, cached at <http://abstreet.s3-website.us-east-2.amazonaws.com/dev/data/input/shared/elevation/UK-dem-50m-4326.tif> and originally processed by [mem48](https://github.com/mem48/)
