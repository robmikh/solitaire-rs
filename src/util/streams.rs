use std::io::{Cursor, Read, Seek, SeekFrom};

use bindings::Windows::Storage::Streams::{IInputStream, IOutputStream, IRandomAccessStream, InputStreamOptions};
use bindings::*;
use bindings::Windows::Win32::Foundation::{E_NOTIMPL, RO_E_CLOSED};
use bindings::Windows::Win32::Storage::StructuredStorage::{IStream, STATSTG, STREAM_SEEK, STREAM_SEEK_CUR};
use windows::implement;

#[implement(Windows::Win32::Storage::StructuredStorage::IStream)]
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
        Self {
            cursor,
            length,
        }
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
            data: Some(ReadOnlyCursorData::copy_from_slice(data))
        }
    }

    fn get_data(&self) -> windows::Result<&ReadOnlyCursorData> {
        if let Some(data) = &self.data {
            Ok(data)
        } else {
            Err(windows::Error::fast_error(RO_E_CLOSED))
        }
    }

    fn get_data_mut(&mut self) -> windows::Result<&mut ReadOnlyCursorData> {
        if let Some(data) = &mut self.data {
            Ok(data)
        } else {
            Err(windows::Error::fast_error(RO_E_CLOSED))
        }
    }

    pub fn Read(&mut self, pv: *mut std::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows::Result<()> {
        let data = self.get_data_mut()?;
        let output = unsafe { std::slice::from_raw_parts_mut(pv as *mut u8, cb as usize) };
        let bytes_read = data.cursor.read(output).unwrap(); // TODO: Translate error
        unsafe { *pcbread = bytes_read as u32 };
        Ok(())
    }

    pub fn Write(&self, pv: *mut std::ffi::c_void, cb: u32) -> windows::Result<u32> {
        E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn Seek(&mut self, dlibmove: i64, dworigin: STREAM_SEEK) -> windows::Result<u64> {
        let data = self.get_data_mut()?;
        let seek = match dworigin {
            STREAM_SEEK_CUR => SeekFrom::Current(dlibmove),
            STREAM_SEEK_END => SeekFrom::End(dlibmove),
            STREAM_SEEK_SET => SeekFrom::Start(dlibmove as u64),
        };
        let position = data.cursor.seek(seek).unwrap(); // TODO: Translate error
        Ok(position)
    }

    pub fn SetSize(&self, libnewsize: u64) -> windows::Result<()> {
        E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn CopyTo(&mut self, stream: &Option<IStream>, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> windows::Result<()> {
        let data = self.get_data_mut()?;
        let mut bytes = vec![0u8; cb as usize];
        let bytes_read = data.cursor.read(&mut bytes).unwrap(); // TODO: Translate error
        unsafe { *pcbread = bytes_read as u64 };
        let bytes_written = unsafe {
            stream.as_ref().unwrap().Write(bytes.as_mut_ptr() as *mut _, bytes_read as u32)?
        };
        unsafe { *pcbwritten = bytes_written as u64 };
        Ok(())
    }

    pub fn Commit(&self, grfcommitflags: u32) -> windows::Result<()> {
        E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn Revert(&self) -> windows::Result<()> {
        E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows::Result<()> {
        E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows::Result<()> {
        E_NOTIMPL.ok()?;
        unimplemented!()
    }

    pub fn Stat(&self, pstatstg: *mut STATSTG, grfstatflags: u32) -> windows::Result<()> {
        // TODO: We probably need this to communicate the size
        unimplemented!()
    }

    pub fn Clone(&self) -> windows::Result<IStream> {
        E_NOTIMPL.ok()?;
        unimplemented!()
    }
}
