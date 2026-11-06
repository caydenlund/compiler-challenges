use std::io;
use std::ops::{Index, IndexMut};

/// An image composed of [r: f32, g: f32, b: f32] pixels
pub struct Image {
    /// The pixel values contained in the image
    pixels: Box<[[f32; 3]]>,
    /// The width of the image
    width: usize,
}

impl Image {
    /// Constructs a new [`Image`] of the given size
    ///
    /// # Panics
    ///
    /// When constructing an image with a width or height of 0
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        assert!(width > 0, "image width must be positive");
        assert!(height > 0, "image height must be positive");
        Self {
            pixels: vec![[0.; 3]; width * height].into_boxed_slice(),
            width,
        }
    }

    /// Gets a reference to the pixel at the given row and column
    #[must_use]
    pub fn get(&self, row: usize, col: usize) -> Option<&[f32; 3]> {
        if col > self.width {
            return None;
        }
        self.pixels.get(row * self.width + col)
    }

    /// Gets a mutable reference to the pixel at the given row and column
    #[must_use]
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut [f32; 3]> {
        if col > self.width {
            return None;
        }
        self.pixels.get_mut(row * self.width + col)
    }

    /// Writes this image's raw color bytes
    ///
    /// # Errors
    ///
    /// Returns [`std::io::Error`] on an I/O error
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn write_raw(&self, mut out: impl io::Write) -> io::Result<()> {
        let mut buf = Vec::with_capacity(3 * self.width);
        for row in self.pixels.chunks_exact(self.width) {
            for [r, g, b] in row {
                buf.push((r * 256.).clamp(0., 255.) as u8);
                buf.push((g * 256.).clamp(0., 255.) as u8);
                buf.push((b * 256.).clamp(0., 255.) as u8);
            }
            out.write_all(&buf)?;
            buf.clear();
        }

        Ok(())
    }

    /// Writes this image in the PPM file format
    ///
    /// # Errors
    ///
    /// Returns [`std::io::Error`] on an I/O error
    pub fn write_ppm(&self, mut out: impl io::Write) -> io::Result<()> {
        writeln!(
            out,
            "P6\n{} {}\n255",
            self.width,
            self.pixels.len() / self.width
        )?;
        self.write_raw(out)
    }
}

impl Index<(usize, usize)> for Image {
    type Output = [f32; 3];

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.pixels[row * self.width + col]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.pixels[row * self.width + col]
    }
}
