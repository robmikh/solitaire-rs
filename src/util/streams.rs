use std::io::{Cursor, Read, Seek, SeekFrom};

use windows as Windows;
use windows::core::{implement, Result};
use windows::Win32::Foundation::RO_E_CLOSED;
use windows::Win32::System::Com::{IStream, STATSTG, STREAM_SEEK, STREAM_SEEK_CUR};
use Windows::Win32::System::Com::{STREAM_SEEK_END, STREAM_SEEK_SET};

#[implement(Windows::Win32::System::Com::IStream)]
pub struct ReadOnlyCursorToStreamWrapper {
    data: Option<ReadOnlyCursorData>,
}

struct ReadOnlyCursorData {
    cursor: Cursor<Vec<u8>>,
    length: u64,
}

impl ReadOnlyCursorData {
    pub fn new(cursor: Cursor<Vec<u8>>) -> Self {
        let length = cursor.get_ref().len() as u64;
        Self { cursor, length }
    }

    pub fn copy_from_slice(slice: &[u8]) -> Self {
        let mut vec = vec![0; slice.len()];
        vec.copy_from_slice(slice);
        let cursor = Cursor::new(vec);
        Self::new(cursor)
    }
}

#[allow(non_snake_case)]
impl ReadOnlyCursorToStreamWrapper {
    pub fn new(data: &[u8]) -> Self {
        Self {
            data: Some(ReadOnlyCursorData::copy_from_slice(data)),
        }
    }

    fn get_data(&self) -> Result<&ReadOnlyCursorData> {
        if let Some(data) = &self.data {
            Ok(data)
        } else {
            Err(windows::core::Error::fast_error(RO_E_CLOSED))
        }
    }

    fn get_data_mut(&mut self) -> Result<&mut ReadOnlyCursorData> {
        if let Some(data) = &mut self.data {
            Ok(data)
        } else {
            Err(windows::core::Error::fast_error(RO_E_CLOSED))
        }
    }

    pub fn Read(&mut self, pv: *mut std::ffi::c_void, cb: u32, pcbread: *mut u32) -> Result<()> {
        let data = self.get_data_mut()?;
        let output = unsafe { std::slice::from_raw_parts_mut(pv as *mut u8, cb as usize) };
        let bytes_read = data.cursor.read(output).unwrap(); // TODO: Translate error
        unsafe { *pcbread = bytes_read as u32 };
        Ok(())
    }

    pub fn Write(&mut self, pv: *const std::ffi::c_void, cb: u32) -> Result<u32> {
        //E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn Seek(&mut self, dlibmove: i64, dworigin: STREAM_SEEK) -> Result<u64> {
        let data = self.get_data_mut()?;
        let seek = match dworigin {
            STREAM_SEEK_CUR => SeekFrom::Current(dlibmove),
            STREAM_SEEK_END => SeekFrom::End(dlibmove),
            STREAM_SEEK_SET => SeekFrom::Start(dlibmove as u64),
            _ => panic!(),
        };
        let position = data.cursor.seek(seek).unwrap(); // TODO: Translate error
        Ok(position)
    }

    pub fn SetSize(&mut self, libnewsize: u64) -> Result<()> {
        //E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn CopyTo(
        &mut self,
        stream: &Option<IStream>,
        cb: u64,
        pcbread: *mut u64,
        pcbwritten: *mut u64,
    ) -> Result<()> {
        let data = self.get_data_mut()?;
        let mut bytes = vec![0u8; cb as usize];
        let bytes_read = data.cursor.read(&mut bytes).unwrap(); // TODO: Translate error
        unsafe { *pcbread = bytes_read as u64 };
        let bytes_written = unsafe {
            stream
                .as_ref()
                .unwrap()
                .Write(bytes.as_mut_ptr() as *mut _, bytes_read as u32)?
        };
        unsafe { *pcbwritten = bytes_written as u64 };
        Ok(())
    }

    pub fn Commit(&mut self, grfcommitflags: u32) -> Result<()> {
        //E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn Revert(&mut self) -> Result<()> {
        //E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn LockRegion(&mut self, liboffset: u64, cb: u64, dwlocktype: u32) -> Result<()> {
        //E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn UnlockRegion(&mut self, liboffset: u64, cb: u64, dwlocktype: u32) -> Result<()> {
        //E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn Stat(&mut self, pstatstg: *mut STATSTG, grfstatflags: u32) -> Result<()> {
        // TODO: We probably need this to communicate the size
        unimplemented!()
    }

    pub fn Clone(&mut self) -> Result<IStream> {
        //E_NOTIMPL.ok()?;
        unimplemented!()
    }
}
