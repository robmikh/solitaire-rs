use std::sync::RwLock;

use windows::core::{implement, Interface, Result};
use windows::Storage::Streams::{
    IBuffer, IBuffer_Impl, IRandomAccessStream, InMemoryRandomAccessStream,
};
use windows::Win32::Foundation::E_NOTIMPL;
use windows::Win32::System::Com::IStream;
use windows::Win32::System::WinRT::{
    CreateStreamOverRandomAccessStream, IBufferByteAccess, IBufferByteAccess_Impl,
};

#[implement(IBuffer, IBufferByteAccess)]
struct MemoryBuffer {
    data: RwLock<Vec<u8>>,
}

impl MemoryBuffer {
    pub fn copy_from_slice(slice: &[u8]) -> Self {
        let mut vec = vec![0; slice.len()];
        vec.copy_from_slice(slice);
        Self {
            data: RwLock::new(vec),
        }
    }
}

impl IBuffer_Impl for MemoryBuffer {
    fn Capacity(&self) -> Result<u32> {
        let data = self.data.read().unwrap();
        Ok(data.len() as u32)
    }

    fn Length(&self) -> Result<u32> {
        let data = self.data.read().unwrap();
        Ok(data.len() as u32)
    }

    fn SetLength(&self, value: u32) -> Result<()> {
        Err(E_NOTIMPL.into())
    }
}

impl IBufferByteAccess_Impl for MemoryBuffer {
    fn Buffer(&self) -> Result<*mut u8> {
        let mut data = self.data.write().unwrap();
        Ok(data.as_mut_ptr())
    }
}

pub fn create_stream_from_bytes(bytes: &[u8]) -> Result<IStream> {
    let buffer: IBuffer = MemoryBuffer::copy_from_slice(bytes).into();
    let memory_stream = InMemoryRandomAccessStream::new()?;
    memory_stream.WriteAsync(buffer)?.get()?;
    let random_access_stream: IRandomAccessStream = memory_stream.cast()?;
    unsafe { CreateStreamOverRandomAccessStream(random_access_stream) }
}
