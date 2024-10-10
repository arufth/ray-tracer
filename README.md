```
██████╗''█████╗'██╗'''██╗''''████████╗██████╗''█████╗''██████╗███████╗██████╗'
██╔══██╗██╔══██╗╚██╗'██╔╝''''╚══██╔══╝██╔══██╗██╔══██╗██╔════╝██╔════╝██╔══██╗
██████╔╝███████║'╚████╔╝''''''''██║'''██████╔╝███████║██║'''''█████╗''██████╔╝
██╔══██╗██╔══██║''╚██╔╝'''''''''██║'''██╔══██╗██╔══██║██║'''''██╔══╝''██╔══██╗
██║''██║██║''██║'''██║''''''''''██║'''██║''██║██║''██║╚██████╗███████╗██║''██║
╚═╝''╚═╝╚═╝''╚═╝'''╚═╝''''''''''╚═╝'''╚═╝''╚═╝╚═╝''╚═╝'╚═════╝╚══════╝╚═╝''╚═╝
```

This project is an implementation of the **Ray Tracing in One Weekend** series by Peter Shirley, adapted to the Rust programming language. The original series guides readers through building a simple ray tracer from scratch over a weekend using C++. This Rust adaptation follows the same concepts and progression but utilizes Rust's features.
<img src="https://raytracing.github.io/images/img-1.23-book1-final.jpg" alt="Final Result" width="100%"/>
<small>This image is from the book series. It is not mine, but this Rust code has the same possibilities as the original.</small>

## About

Ray tracing is a rendering technique used to generate realistic images by tracing the path of light as pixels in an image plane and simulating how it interacts with virtual objects. **Ray Tracing in One Weekend** provides a hands-on introduction to ray tracing, making it accessible even to those with minimal experience in computer graphics.

## Current Phase

This project is currently in phase 1, following the concepts introduced in the first book of the **Ray Tracing in One Weekend** series.

## Usage

To build and run the Rust Ray Tracer, follow these steps:

1. **Clone the repository**:

   ```sh
   git clone https://github.com/carlitosdummy/ray-tracer
   cd ray-tracer
   ```

2. **Run the project**:

   ```sh
   cargo run --release
   ```

3. **Output**:
   The rendered image will be saved as [`output.ppm`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fdummy%2FDeveloper%2Frust%2Fray-tracer%2Foutput.ppm%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%5D "/Users/dummy/Developer/rust/ray-tracer/output.ppm") in the project directory. You can view this image using an image viewer that supports the PPM format.

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system before running these commands.

## Rendering Options

You can configure various rendering options by modifying the [`Camera`](command:_github.copilot.openSymbolFromReferences?%5B%7B%22%24mid%22%3A1%2C%22path%22%3A%22%2FUsers%2Fdummy%2FDeveloper%2Frust%2Fray-tracer%2Fsrc%2Fcamera.rs%22%2C%22scheme%22%3A%22file%22%7D%2C%7B%22line%22%3A14%2C%22character%22%3A0%7D%5D "src/camera.rs") struct in [`src/main.rs`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fdummy%2FDeveloper%2Frust%2Fray-tracer%2Fsrc%2Fmain.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%5D "/Users/dummy/Developer/rust/ray-tracer/src/main.rs"). Here are some of the options you can set:

- **Aspect Ratio**: Set the aspect ratio of the rendered image.

  ```rust
  cam.aspect_ratio = 16.0 / 9.0;
  ```

- **Image Width**: Set the width of the rendered image.

  ```rust
  cam.image_width = 1200;
  ```

- **Samples per Pixel**: Set the number of samples per pixel for anti-aliasing.

  ```rust
  cam.samples_per_pixel = 10;
  ```

- **Max Depth**: Set the maximum depth for ray tracing recursion.

  ```rust
  cam.max_depth = 50;
  ```

- **Vertical Field of View (VFOV)**: Set the vertical field of view.

  ```rust
  cam.vfov = 20.0;
  ```

- **Camera Position and Orientation**: Set the camera position (`lookfrom`), target (`lookat`), and up vector (`vup`).

  ```rust
  cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
  cam.lookat = Point3::new(0.0, 0.0, 0.0);
  cam.vup = Vector3::new(0.0, 1.0, 0.0);
  ```

- **Defocus Angle**: Set the defocus angle for depth of field effects.

  ```rust
  cam.defocus_angle = 0.6;
  ```

- **Focus Distance**: Set the focus distance for depth of field effects.

  ```rust
  cam.focus_dist = 10.0;
  ```

For more details, refer to the [`Camera`](command:_github.copilot.openSymbolFromReferences?%5B%7B%22%24mid%22%3A1%2C%22path%22%3A%22%2FUsers%2Fdummy%2FDeveloper%2Frust%2Fray-tracer%2Fsrc%2Fcamera.rs%22%2C%22scheme%22%3A%22file%22%7D%2C%7B%22line%22%3A14%2C%22character%22%3A0%7D%5D "src/camera.rs") implementation in [`src/camera.rs`](command:_github.copilot.openRelativePath?%5B%7B%22scheme%22%3A%22file%22%2C%22authority%22%3A%22%22%2C%22path%22%3A%22%2FUsers%2Fdummy%2FDeveloper%2Frust%2Fray-tracer%2Fsrc%2Fcamera.rs%22%2C%22query%22%3A%22%22%2C%22fragment%22%3A%22%22%7D%5D "/Users/dummy/Developer/rust/ray-tracer/src/camera.rs").

## Resources

- [Original "Ray Tracing in One Weekend" series](https://raytracing.github.io/)
- [Rust programming language](https://www.rust-lang.org/)
