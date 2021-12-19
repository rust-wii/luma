//! Contains functions for accessing the USB Gecko.

use crate::exi::{Channel, Clock, Cr, Exi};
use alloc::vec::Vec;

#[derive(Debug)]
pub enum Error {
    UsbGeckoNotFound,
    SendImpossible,
    ReceiveImpossible,
}

/// Struct which represents an usable USB Gecko.
pub struct UsbGecko<'a> {
    channel: Channel<'a>,
}

impl<'a> UsbGecko<'a> {
    /// Try accessing a plugged in USB Gecko, returns Error::UsbGeckoNotFound if none is currently
    /// plugged in.
    pub fn new(exi: &'a Exi) -> Result<UsbGecko<'a>, Error> {
        let channel = exi.get_channel(1);
        let gecko = UsbGecko { channel };
        if !gecko.check_usb_gecko() {
            return Err(Error::UsbGeckoNotFound);
        }
        Ok(gecko)
    }

    fn query_usb_gecko(&self, query: u16) -> u16 {
        self.channel.select_device(0, Clock::CLOCK_32MHZ);
        self.channel.write_data((query as u32) << 16);
        self.channel
            .write_cr(Cr::TLEN16 | Cr::RW | Cr::IMMEDIATE | Cr::TSTART);
        self.channel.wait_for_completion();
        (self.channel.read_data() >> 16) as u16
    }

    fn check_usb_gecko(&self) -> bool {
        let query = 0x9000;
        let ret = self.query_usb_gecko(query);
        ret == 0x0470
    }

    fn receive_byte(&self) -> Result<u8, Error> {
        let query = 0xa000;
        let ret = self.query_usb_gecko(query);
        if ret & 0x0800 == 0 {
            return Err(Error::ReceiveImpossible);
        }
        Ok((ret & 0xff) as u8)
    }

    fn send_byte(&self, byte: u8) -> Result<(), Error> {
        let query = 0xb000 | ((byte as u16) << 4);
        let ret = self.query_usb_gecko(query);
        if ret & 0x0400 == 0 {
            return Err(Error::SendImpossible);
        }
        Ok(())
    }

    fn can_send(&self) -> bool {
        let query = 0xc000;
        let ret = self.query_usb_gecko(query);
        ret & 0x0400 != 0
    }

    fn can_receive(&self) -> bool {
        let query = 0xd000;
        let ret = self.query_usb_gecko(query);
        ret & 0x0400 != 0
    }

    /// Receive one or more bytes from the USB Gecko and returns them in a Vec.  This function
    /// returns Error::ReceiveImpossible if no bytes are available.
    pub fn receive(&self) -> Result<Vec<u8>, Error> {
        let mut data = Vec::new();
        loop {
            if !self.can_receive() {
                break;
            }
            let byte = self.receive_byte()?;
            data.push(byte);
        }
        if data.is_empty() {
            return Err(Error::ReceiveImpossible);
        }
        Ok(data)
    }

    /// Send a slice of bytes to the USB Gecko.  This function returns Error::SendImpossible if no
    /// byte could be transmitted despite the device saying we could, it otherwise blocks until all
    /// bytes from the slice have been transmitted.
    pub fn send(&self, mut data: &[u8]) -> Result<(), Error> {
        loop {
            if data.is_empty() {
                break;
            }
            if !self.can_send() {
                continue;
            }
            let byte = data[0];
            data = &data[1..];
            self.send_byte(byte)?;
        }
        Ok(())
    }
}
