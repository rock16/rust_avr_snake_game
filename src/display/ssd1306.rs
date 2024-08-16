#![no_std]
// implement driver for ssd1306.rs
//use embedded_hal::prelude::*;
use embedded_io::{Write, Read, ErrorType};
use embedded_hal::i2c::{Error, I2c};
use core::result::Result;

const SSD1306_ADDRESS: u8 = 0x3c;

pub struct Ssd1306<I2C>{
    i2c: I2C,
    width: u8,
    height: u8,
    buffer: [u8; 1024], // buffer size is based on display dimension.
}

impl<I2C> Ssd1306<I2C>
where
    I2C: I2c + Write,
{
    pub fn new(mut i2c: I2C, width: u8, height: u8) -> Self{
        // Initialize the OLED, including setting up display parameters
        // ...
        Self {
            i2c,
            width,
            height,
            buffer: [0; 1024]
        }
    }

    fn init(&mut self) -> Result<(), I2C::Error> {
        // Reset display (optional)
        // ...

        self.command(0xAE)?; // Display off
        self.command(0x00)?; // Set lower column start address
        self.command(0x10)?; // Set higher column start address
        self.command(0x40)?; // Set display start line
        self.command(0x81)?; // Set contrast control
        self.command(0xCF)?; // Set default contrast
        // ... other initialization commands
        self.command(0xAF)?; // Display on

        Ok(())
    }

    // Function to send a command to the OLED
    fn command(&mut self, cmd: u8) -> Result<(), I2C::Error> {
        let mut buf = [0x00, cmd]; // Combine command mode and data
        self.i2c.write(SSD1306_ADDRESS, &buf)?;
        Ok(())
    }

    fn set_column_address(&mut self, start_column: u8, end_column: u8) -> Result<(), I2C::Error> {
        self.command(0x21)?; // Set column address
        self.command(start_column as u8)?;
        self.command(end_column as u8)?;
        Ok(())
    }

    fn set_page_address(&mut self, start_page: u8) -> Result<(), I2C::Error> {
        self.command(0xB0 + start_page)?;
        Ok(())
    }

    pub fn draw_pixel(&mut self, x: u8, y: u8, color: bool) {
        if x >= self.width || y >= self.height {
            return;
        }

        let byte_index = x + y / 8 * self.width;
        let bit_index = y % 8;

        if color {
            self.buffer[byte_index] |= 1 << bit_index;
        } else {
            self.buffer[byte_index] &= !(1 << bit_index);
        }
    }

    pub fn display(&mut self) -> Result<(), I2C::Error> {
        self.set_column_address(0, self.width - 1)?;
        self.set_page_address(0)?;

        // Send the buffer data to the OLED
        for page in 0..8 {
            self.command(0x40 + page)?; // Set page address

            // Ensure we're not accessing the buffer out of bounds
            let buffer_start = page * self.width as usize;
            let buffer_end = buffer_start + self.width as usize;
            let buffer_slice = &self.buffer[buffer_start..buffer_end];

            for byte in buffer_slice {
                self.i2c.send_byte(*byte)?;
            }
        }

        Ok(())
    }

    pub fn draw_line(&mut self, x0: u8, y0: u8, x1: u8, y1: u8, color: bool) {
        let mut dx = (x1 as i16) - (x0 as i16);
        let mut dy = (y1 as i16) - (y0 as i16);
        let mut sx = if dx > 0 { 1 } else { -1 };
        let mut sy = if dy > 0 { 1 } else { -1 };
        dx = dx.abs();
        dy = dy.abs();
        let mut err = dx - dy;

        let mut x = x0 as i16;
        let mut y = y0 as i16;

        while (x as i16) != (x1 as i16) || (y as i16) != (y1 as i16) {
            self.draw_pixel(x as u8, y as u8, color);
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
        self.draw_pixel(x as u8, y as u8, color);
    }

}