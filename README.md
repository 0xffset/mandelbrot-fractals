
<h1 align="center">
  <br>
  <a href="https://mandelbrot-fractals.netlify.app/"><img src="/docs/logo.png" alt="mandelbrot-fractals" width="200"></a>
  <br>
  madenlbrot-fractals
  <br>
</h1>

<h4 align="center">A collection of Mandelbrot fractals visualizations 



<p align="center">
 <a href="#what-is">What is Mandelbrot's set</a> •
  <a href="#key-features">Key Features</a> •
  <a href="#how-to-use">How To Use</a> •
  <a href="#credits">Credits</a> •
  <a href="#license">License</a>
</p>


<img src="/docs/demo.gif" width="400" height="400" />

## What is Mandelbrot set?
The core formula for Mandelbrot set is a simple quadratic iteration: $z_n+1 = z_n^2 + c$, where $z_n$ is a complex number, and $c$ is complex constant. The Mandelbrot set is a collection of complex numbers $c$ for which the sequence $z_n$ remains bounded (does not diverge to infinity) when starting with $z_0 = 0$. 

Here's a more detail brakdown: 
 - $z_n$: Represents a complex number in the iteration, evolving from $z_0=0$.
 - $c$: A complex constant parameter that defines a specific point in the complex plane.
 - $z_n+1$: The next value in the sequence, calculated by squaring the previous vlaue ($z_n$) and adding the constant $c$. 
## Key Features

* Precise Positioning
  - Adjust the view with Position X and Position Y sliders or input fields for fine control over the Mandelbrot set location.
* Zoom Control
  - Modify the Size parameter to zoom into specific regions of the fractal with high precision.
* Iteration Depth
  - Customize the Iterations to balance rendering speed and detail depth for complex visuals.
* Sampling Quality
  - Set the Samples value to apply anti-aliasing and improve image quality for smoother rendering.
* Canvas Dimensions
  - Change Width and Height to customize the resolution of the rendered image.
* Multithreading Support*
  - Use the Threads setting to leverage multicore CPUs for faster rendering performance.
* Paint Modes
  - Select from different Paint Modes such as Grayscale (others may include Color, Smooth, etc., if implemented) for different artistic effects.
* Image Exporting
  - Ideal for generating high-resolution PNGs for further analysis or creative use.
> **⚠️WARNING**
> Multithreading Support could increase the overall workload and heat generation of you CPU due multiple cores processing tasks.
## How To Use

To clone and run this application, you'll need [Rust WASM](https://rustwasm.github.io/book/introduction.html), [ReactJS](https://react.dev/) and [npm](http://npmjs.com) installed on your computer. From your command line:

```bash
# Clone this repository
$ git clone https://github.com/0xffset/mandelbrot-fractals

# Go into the frontend source code
$ cd www

# Install dependencies
$ npm install

# Complie the rust wasm backend 
$ cd ../
$ wasm-pack build

# Run the app
$ cd www/
$ npm run start
```



## Credits

This software uses the following open source packages:

- [Rust and WebAssembly](https://rustwasm.github.io/book/)
- [ReactJS](https://react.dev/)

## License

MIT
