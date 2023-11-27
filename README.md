# elevation

Explore options for reading elevation data in pure Rust

- For A/B Street's super simple use, just calculate start and end elevation given a LineString input (in WGS84). The interface is just looking up a single elevation value at a time!
	- Do bilinear interpolation at least
- How can we validate the results?
	- Generate an image, or run through a contour algorithm?
- Can we parallelize?
	- Decoding is mutable, so seemingly no
	- How expensive is it memory-wise to load one of these in each thread?
	- Just use the bbox bounds to generate a benchmark
- Do we need to support more CRS or can we just transform stuff?
- What should we do for contour cases? (Probably a different approach)
- Should this library exist at all, or should we just add some helper methods upstream to georaster?
