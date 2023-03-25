# Ray Tracing in One Weekend, In Rust

![Sample Render](images/showcase.png)

This repository is a CPU-bound ray tracing library based on Peter Shirley's book [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) and the subsequent books in the series.

It adds some additional features such as parallel, tiled rendering. It further makes changes from the books where possible to achieve more idiomatic Rust code.

# Features

* Materials
  * Lambertians
  * Dialectrics
  * Metals
  * Extensible Material trait representation.
  * Isotropic
* Textures
  * Image textures
  * Procedural textures
  * Extensible Texture trait representation.
* Geometry
  * Constant density convex mediums
  * Implicit surfaces (e.g. spheres, rectangles)
  * Motion Blur
* Performance
  * BVH (Bounding Volume Hierarchy) implementation for fast ray collisions.
  * Multi-threaded, tiled rendering
* Camera
  * Depth of Field
  * Shutter Speed

# Usage

For more details on how to use this crate, run `cargo doc --open` in the cloned repository.

The binary provides a command line interface to rendering sample scenes. To install, while in the cloned repository, use `cargo install --path .`. Then use `shimmer --help` for more informtion. Or, skip installation and run `cargo run -- --help`.

# Sample Renders

![Constant Density Mediums](images/smoke.png)

*Constant Density Mediums*

![Motion Blur](images/motion_blur.png)

*Motion blur*

# Acknowledgements

This repository is based on Peter Shirley's book [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html). I will readily recommend this book to anyone interested in computer graphics, along with his other books [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html) and [_Ray Tracing: The Rest of Your Life_](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html).

# A note on the "Shimmer" moniker and further development

During development of this repository, I had been referring to the project as "Shimmer", and that's the name of the project in cargo.toml. The plan was to develop this repository into a more fully-featured ray tracing library called Shimmer to be deployed to crates.io.

I still have plans to develop Shimmer, but that may or may not live in this repository. I may implement Shimmer as more closely aligned with the [PBRT](https://pbrt.org/) project's architecture. There's already a great Rust implementation of PBRT called [rs-pbrt](https://github.com/wahn/rs_pbrt) which (at the time of writing) is feature complete matching PBRT v3. But, I still have tentative plans to develop Shimmer for educational purposes. Shimmer would also be free to deviate from or expand on PBRT, where rs-pbrt purely aims to be a Rusty counterpart to the project.

So, for now, this repository will be "a Rust implementation of Peter Shirley's books, plus some additional features as I care to develop them". I still plan to develop the Shimmer ray tracing project, but that may be in a new repository due to differing architecture decisions.