use futures::stream::FuturesUnordered;
use futures::StreamExt;
use js_sys::Promise;
use num_complex::Complex;
use num_complex::ComplexFloat;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::window;
use web_sys::{CanvasRenderingContext2d, ImageData, Performance};

// When the `console_error_panic_hook` feature is enabled, we can call the
// `set_panic_hook` function to get better error messages if we ever panic.
#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

// Helper macro for console logging
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {
        web_sys::console::log_1(&format!($($arg)*).into());
    };
}

#[derive(Clone)]
struct MandelbrotWorker {
    width: u32,
    height: u32,
    pos_x: f64,
    pos_y: f64,
    size: f64,
    max_iterations: u32,
    samples: u32,
    render_time: f64,
    ctx: CanvasRenderingContext2d,
    paint_mode: PaintMode,
    performance: Performance,
    // all fields needed for pixel rendering (like zoom, position, etc.)
}

fn show_loading_modal() {
    if let Some(document) = window().unwrap().document() {
        if let Some(loading_el) = document.get_element_by_id("loading-modal") {
            loading_el.set_class_name("modal show"); // Ensure this class makes it visible
        }
    }
}

fn hide_loading_modal() {
    if let Some(document) = window().unwrap().document() {
        if let Some(loading_el) = document.get_element_by_id("loading-modal") {
            loading_el.set_class_name("modal hide"); // Ensure this class hides it
        }
    }
}

impl MandelbrotWorker {
    fn calculate_pixel(&self, x: u32, y: u32) -> u32 {
        if self.samples <= 1 {
            // No supersampling
            let cx = self.pos_x + (x as f64 / self.width as f64 - 0.5) * self.size;
            let cy = self.pos_y
                + (y as f64 / self.height as f64 - 0.5)
                    * self.size
                    * (self.height as f64 / self.width as f64);
            return self.calculate_point(cx, cy);
        }

        // With supersampling
        let mut total = 0;
        let step = 1.0 / (self.samples as f64 + 1.0);

        for sx in 0..self.samples {
            for sy in 0..self.samples {
                let cx = self.pos_x
                    + ((x as f64 + step * (sx as f64 + 1.0)) / self.width as f64 - 0.5) * self.size;
                let cy = self.pos_y
                    + ((y as f64 + step * (sy as f64 + 1.0)) / self.height as f64 - 0.5)
                        * self.size
                        * (self.height as f64 / self.width as f64);
                total += self.calculate_point(cx, cy);
            }
        }

        total / (self.samples * self.samples)
    }
    // Color mapping functions
    fn map_color(&self, iterations: u32) -> [u8; 4] {
        if iterations >= self.max_iterations {
            return [0, 0, 0, 255]; // Black for points in the set
        }

        match self.paint_mode {
            PaintMode::Grayscale => {
                let v = 255 - (iterations as f64 / self.max_iterations as f64 * 255.0) as u8;
                [v, v, v, 255]
            }
            PaintMode::HSL1 => {
                // Rainbow hue based on iteration count
                let hue = (iterations as f64 / self.max_iterations as f64 * 360.0) as u16;
                self.hsl_to_rgb(hue, 100, 50)
            }
            PaintMode::HSL2 => {
                // Logarithmic mapping for more detail
                let log_iterations = (iterations as f64).log(self.max_iterations as f64);
                let hue = ((1.0 - log_iterations) * 240.0) as u16;
                self.hsl_to_rgb(hue, 100, 50)
            }
            PaintMode::HSL3 => {
                // Smooth coloring
                let log_zn = 0.0;
                let nu =
                    (iterations as f64 + 1.0 - log_zn.log2().log2()) / self.max_iterations as f64;
                let hue = (360.0 * nu) as u16 % 360;
                self.hsl_to_rgb(hue, 100, 50)
            }
            PaintMode::RGB1 => {
                // RGB banding
                let r = (iterations as f64 * 9.0).sin().abs() * 255.0;
                let g = (iterations as f64 * 3.0).sin().abs() * 255.0;
                let b = (iterations as f64 * 5.0).sin().abs() * 255.0;
                [r as u8, g as u8, b as u8, 255]
            }
            PaintMode::FIRE => {
                let t = iterations as f64 / self.max_iterations as f64;

                let r = (9.0 * (1.0 - t) * t * t * t * 255.0).min(255.0);
                let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0).min(255.0);
                let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0).min(255.0);

                [r as u8, g as u8, b as u8, 255]
            }
        }
    }

    // HSL to RGB conversion
    fn hsl_to_rgb(&self, h: u16, s: u8, l: u8) -> [u8; 4] {
        let h = (h % 360) as f64 / 60.0;
        let s = s as f64 / 100.0;
        let l = l as f64 / 100.0;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = match h as u8 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        [
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
            255,
        ]
    }

    // Calculate single mandelbrot pixel
    fn calculate_point(&self, cx: f64, cy: f64) -> u32 {
        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0.0, 0.0);
        let mut iterations = 0;

        while iterations < self.max_iterations && z.norm_sqr() <= 4.0 {
            z = z * z + c;
            iterations += 1;
        }

        iterations
    }
}

// Paint modes
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum PaintMode {
    Grayscale,
    HSL1,
    HSL2,
    HSL3,
    RGB1,
    FIRE,
}

// Main Mandelbrot struct
#[wasm_bindgen]
pub struct Mandelbrot {
    width: u32,
    height: u32,
    pos_x: f64,
    pos_y: f64,
    size: f64,
    max_iterations: u32,
    samples: u32,
    paint_mode: PaintMode,
    render_time: f64,
    ctx: CanvasRenderingContext2d,
    performance: Performance,
}

#[wasm_bindgen]
impl Mandelbrot {
    #[wasm_bindgen(constructor)]
    pub fn new(
        canvas_id: &str,
        width: u32,
        height: u32,
        pos_x: f64,
        pos_y: f64,
        size: f64,
        max_iterations: u32,
        samples: u32,
        paint_mode: PaintMode,
    ) -> Result<Mandelbrot, JsValue> {
        // Set up panic hook for better error messages
        #[cfg(feature = "console_error_panic_hook")]
        set_panic_hook();

        // Get canvas context
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("Element is not a canvas"))?;

        canvas.set_width(width);
        canvas.set_height(height);

        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        // Get performance object for timing
        let performance = web_sys::window()
            .unwrap()
            .performance()
            .expect("performance should be available");

        Ok(Mandelbrot {
            width,
            height,
            pos_x,
            pos_y,
            size,
            max_iterations,
            samples,
            paint_mode,
            render_time: 0.0,
            ctx,
            performance,
        })
    }

    // Set methods for parameters
    pub fn set_pos_x(&mut self, pos_x: f64) {
        self.pos_x = pos_x;
    }

    pub fn set_pos_y(&mut self, pos_y: f64) {
        self.pos_y = pos_y;
    }

    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }

    pub fn set_max_iterations(&mut self, max_iterations: u32) {
        self.max_iterations = max_iterations;
    }

    pub fn set_samples(&mut self, samples: u32) {
        self.samples = samples;
    }

    pub fn set_paint_mode(&mut self, paint_mode: PaintMode) {
        self.paint_mode = paint_mode;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let canvas = self.ctx.canvas().unwrap();
        canvas.set_width(width);
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let canvas = self.ctx.canvas().unwrap();
        canvas.set_height(height);
    }

    pub fn get_render_time(&self) -> f64 {
        self.render_time
    }

    // Calculate single mandelbrot pixel
    fn calculate_point(&self, cx: f64, cy: f64) -> u32 {
        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0.0, 0.0);
        let mut iterations = 0;

        while iterations < self.max_iterations && z.norm_sqr() <= 4.0 {
            z = z * z + c;
            iterations += 1;
        }

        iterations
    }

    // Calculate with supersampling
    fn calculate_pixel(&self, x: u32, y: u32) -> u32 {
        if self.samples <= 1 {
            // No supersampling
            let cx = self.pos_x + (x as f64 / self.width as f64 - 0.5) * self.size;
            let cy = self.pos_y
                + (y as f64 / self.height as f64 - 0.5)
                    * self.size
                    * (self.height as f64 / self.width as f64);
            return self.calculate_point(cx, cy);
        }

        // With supersampling
        let mut total = 0;
        let step = 1.0 / (self.samples as f64 + 1.0);

        for sx in 0..self.samples {
            for sy in 0..self.samples {
                let cx = self.pos_x
                    + ((x as f64 + step * (sx as f64 + 1.0)) / self.width as f64 - 0.5) * self.size;
                let cy = self.pos_y
                    + ((y as f64 + step * (sy as f64 + 1.0)) / self.height as f64 - 0.5)
                        * self.size
                        * (self.height as f64 / self.width as f64);
                total += self.calculate_point(cx, cy);
            }
        }

        total / (self.samples * self.samples)
    }

    // Color mapping functions
    fn map_color(&self, iterations: u32) -> [u8; 4] {
        if iterations >= self.max_iterations {
            return [0, 0, 0, 255]; // Black for points in the set
        }

        match self.paint_mode {
            PaintMode::Grayscale => {
                let v = 255 - (iterations as f64 / self.max_iterations as f64 * 255.0) as u8;
                [v, v, v, 255]
            }
            PaintMode::HSL1 => {
                // Rainbow hue based on iteration count
                let hue = (iterations as f64 / self.max_iterations as f64 * 360.0) as u16;
                self.hsl_to_rgb(hue, 100, 50)
            }
            PaintMode::HSL2 => {
                // Logarithmic mapping for more detail
                let log_iterations = (iterations as f64).log(self.max_iterations as f64);
                let hue = ((1.0 - log_iterations) * 240.0) as u16;
                self.hsl_to_rgb(hue, 100, 50)
            }
            PaintMode::HSL3 => {
                // Smooth coloring
                let log_zn = 0.0;
                let nu =
                    (iterations as f64 + 1.0 - log_zn.log2().log2()) / self.max_iterations as f64;
                let hue = (360.0 * nu) as u16 % 360;
                self.hsl_to_rgb(hue, 100, 50)
            }
            PaintMode::RGB1 => {
                // RGB banding
                let r = (iterations as f64 * 9.0).sin().abs() * 255.0;
                let g = (iterations as f64 * 3.0).sin().abs() * 255.0;
                let b = (iterations as f64 * 5.0).sin().abs() * 255.0;
                [r as u8, g as u8, b as u8, 255]
            }
            PaintMode::FIRE => {
                let t = iterations as f64 / self.max_iterations as f64;

                let r = (9.0 * (1.0 - t) * t * t * t * 255.0).min(255.0);
                let g = (15.0 * (1.0 - t) * (1.0 - t) * t * t * 255.0).min(255.0);
                let b = (8.5 * (1.0 - t) * (1.0 - t) * (1.0 - t) * t * 255.0).min(255.0);

                [r as u8, g as u8, b as u8, 255]
            }
        }
    }

    // HSL to RGB conversion
    fn hsl_to_rgb(&self, h: u16, s: u8, l: u8) -> [u8; 4] {
        let h = (h % 360) as f64 / 60.0;
        let s = s as f64 / 100.0;
        let l = l as f64 / 100.0;

        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let m = l - c / 2.0;

        let (r, g, b) = match h as u8 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        [
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
            255,
        ]
    }

    // Standard render method
    // pub fn render(&mut self) -> Result<(), JsValue> {
    //     let start_time = self.performance.now();

    //     // Create image data
    //     let mut data = vec![0u8; (self.width * self.height * 4) as usize];

    //     // Calculate all pixels
    //     for y in 0..self.height {
    //         for x in 0..self.width {
    //             let index = ((y * self.width + x) * 4) as usize;
    //             let iterations = self.calculate_pixel(x, y);
    //             let color = self.map_color(iterations);

    //             data[index] = color[0]; // R
    //             data[index + 1] = color[1]; // G
    //             data[index + 2] = color[2]; // B
    //             data[index + 3] = color[3]; // A
    //         }
    //     }

    //     // Create ImageData and put on canvas
    //     let data = ImageData::new_with_u8_clamped_array_and_sh(
    //         wasm_bindgen::Clamped(&data),
    //         self.width,
    //         self.height,
    //     )?;

    //     self.ctx.put_image_data(&data, 0.0, 0.0)?;

    //     // Record render time
    //     self.render_time = (self.performance.now() - start_time) / 1000.0; // Convert to seconds

    //     Ok(())
    // }

    // Chunked rendering method that yields control back to the browser
    // Not truly parallel but simulates the effect of multithreading for UI responsiveness
    pub async fn render_parallel(&mut self, chunks: usize) -> Result<(), JsValue> {
        show_loading_modal();
        let start_time = self.performance.now();

        let width = self.width;
        let height = self.height;
        let chunk_size = (height / chunks as u32).max(1);

        let mut tasks = FuturesUnordered::new();

        for chunk in 0..chunks as u32 {
            let start_y = chunk * chunk_size;
            let end_y = if chunk == chunks as u32 - 1 {
                height
            } else {
                (chunk + 1) * chunk_size
            };

            let width = self.width;
            let renderer = MandelbrotWorker {
                width: self.width,
                height: self.height,
                pos_x: self.pos_x,
                pos_y: self.pos_y,
                size: self.size,
                max_iterations: self.max_iterations,
                samples: self.samples,
                render_time: self.render_time,
                ctx: self.ctx.clone(),
                paint_mode: self.paint_mode,
                performance: self.performance.clone(),
            };

            tasks.push(async move {
                let mut sub_data = vec![0u8; ((end_y - start_y) * width * 4) as usize];

                for y in start_y..end_y {
                    for x in 0..width {
                        let iterations = renderer.calculate_pixel(x, y);
                        let color = renderer.map_color(iterations);
                        let local_y = y - start_y;
                        let index = ((local_y * width + x) * 4) as usize;
                        sub_data[index] = color[0];
                        sub_data[index + 1] = color[1];
                        sub_data[index + 2] = color[2];
                        sub_data[index + 3] = color[3];
                    }
                }

                (start_y, sub_data)
            });
        }

        // Merge results
        let mut final_data = vec![0u8; (width * height * 4) as usize];
        while let Some((start_y, sub_data)) = tasks.next().await {
            let start_index = (start_y * width * 4) as usize;
            final_data[start_index..start_index + sub_data.len()].copy_from_slice(&sub_data);
        }

        // Yield once more to ensure UI can update
        JsFuture::from(Promise::resolve(&JsValue::from(0))).await?;

        // Create ImageData and put it on canvas
        let image_data = ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&final_data),
            width,
            height,
        )?;

        self.ctx.put_image_data(&image_data, 0.0, 0.0)?;

        // Record render time
        self.render_time = (self.performance.now() - start_time) / 1000.0;
        hide_loading_modal();
        Ok(())
    }

    // Save to PNG
    pub fn save_to_png(&self) -> Result<(), JsValue> {
        let canvas = self.ctx.canvas().unwrap();
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // Create a temporary link element
        let a = document
            .create_element("a")?
            .dyn_into::<web_sys::HtmlAnchorElement>()?;

        // Get the canvas data URL
        let data_url = canvas.to_data_url()?;

        // Set attributes for downloading
        a.set_href(&data_url);
        a.set_download("mandelbrot.png");

        // Append to document, click, and remove
        document.body().unwrap().append_child(&a)?;
        a.click();
        document.body().unwrap().remove_child(&a)?;

        Ok(())
    }
}
