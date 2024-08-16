#![no_std]

use avr_hal_generic::i2c::twi_status::{TW_MR_DATA_ACK, TW_MT_DATA_ACK};
use embedded_hal::i2c::{Operation, SevenBitAddress, ErrorType};
//use embedded_hal::blocking::i2c::{I2c, Write, Read};

const TWINT: u8 = 7;
const TWEA: u8 = 6;
const TWSTA: u8 = 5;
const TWSTO: u8 = 4;
const TWWC: u8 = 3;
const TWEN: u8 = 0;
pub struct LocalI2C {
    // TWI registers
    twbr: *mut u8,
    twcr: *mut u8,
    twdr: *mut u8,
    twsr: *mut u8,
    // still thinking about including this registers or not
    //twint: *mut u8,
}

#[derive(Debug)]
pub enum I2cError{
    // state the error types suck as AckError
    NackError,
}

impl LocalI2C {
    pub fn new(/* .. */) -> Self{
        // Initialize TWI registers
        Self {
            // ..
        }
    }

   fn start(&mut self) -> Result<(), ()>{
       unsafe {
           // create the start condition
           let twcr_ptr = self.twcr;
           *twcr_ptr |= 1 <<TWSTA | 1 <<TWEN;

           // wait for the start condition to complete
           while *twcr_ptr & (1 << TWINT) == 0 {}

           // check for errors
           let status = *self.twsr;
           if status != 0x08{ // start condition transmitted
               return Err(())
           }
           Ok(())
       }
   }


    fn stop(&mut self)-> Result<(), ()>{
        unsafe {
            // create stop condition
            let twcr_ptr = self.twcr as *mut u8;
            *twcr_ptr |= 1 << TWINT | 1 << TWSTO | 1 << TWEN;

            // wait for stop condition to complete
            while *twcr_ptr & (1 << TWSTO) != 0 {}
            Ok(())
        }
    }


    fn send_byte(&mut self, data: u8) -> Result<(), Self::Error> {
        unsafe {
            *self.twdr = data;
            let twcr_ptr = self.twcr;
            *twcr_ptr |= (1 << TWINT) | (1 << TWEN);
            while !(*twcr_ptr & (1 << TWINT)) {}
            let status = *self.twsr;
            if (status & 0xF8) != TW_MT_DATA_ACK {
                return Err(I2cError::NackError);
            }
            Ok(())
        }

    }
    fn receive_byte(&mut self, ack: bool) -> Result<u8, ()> {
        unsafe {
            // Set TWINT, TWEN, and TWEA bits in TWCR
            let twcr_ptr = self.twcr as *mut u8;
            *twcr_ptr |= (1 << TWINT) | (1 << TWEN) | (if ack { 0 } else { 1 << TWEA });

            // Wait for TWINT to be set
            while (*twcr_ptr & (1 << TWINT)) == 0 {}

            // Check for NACK
            let status = *self.twsr as u8;
            if (status & 0xF8) != TW_MR_DATA_ACK {
                return Err(());
            }

            Ok(*self.twdr as u8)
        }
    }

    fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), I2cError>{
        // Implement read operation using TWI registers
        Ok(())
    }
    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), I2cError>{
        // ... start condition
        self.start().expect("TODO: panic message");
        // ... send device address with write bit
        self.send_byte((addr << 1) & 0xFE)?;
        // ... for each byte in bytes
        for byte in bytes{
            // send byte
            self.send_byte(*byte).unwrap()
        }
        //      check for ack
        // ...  stop condition
        self.stop();
        Ok(())
    }

}

impl ErrorType for LocalI2C { type Error = (); }
impl ErrorType for I2cError{
    type Error = ();
}


impl embedded_hal::i2c::I2c for LocalI2C {
    fn read(&mut self, address: SevenBitAddress, read: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn write(&mut self, address: SevenBitAddress, write: &[u8]) -> Result<(), Self::Error> {
        // ... start condition
        self.start();
        // ... send device address with write bit
        self.send_byte((address << 1) & 0xFE)?;
        // ... for each byte in bytes
        for byte in write{
            // send byte
            self.send_byte(*byte).unwrap()
        }
        //      check for ack
        // ...  stop condition
        self.stop();
        Ok(())
    }

    fn write_read(&mut self, address: SevenBitAddress, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
        todo!()
    }

    fn transaction(&mut self, address: SevenBitAddress, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
        todo!()
    }
}

