pub mod streams {
    use std::io::{Cursor, Read, Seek, SeekFrom};
    use windows as Windows;
    use windows::core::{implement, Result};
    use windows::Storage::Streams::{
        IInputStream, IOutputStream, IRandomAccessStream, InputStreamOptions,
    };
    use windows::Win32::Foundation::{E_NOTIMPL, RO_E_CLOSED};
    use windows::Win32::System::Com::{IStream, STATSTG, STREAM_SEEK, STREAM_SEEK_CUR};
    impl ::core::convert::From<ReadOnlyCursorToStreamWrapper> for Windows::Win32::System::Com::IStream {
        fn from(implementation: ReadOnlyCursorToStreamWrapper) -> Self {
            let com = ReadOnlyCursorToStreamWrapper_box::new(implementation);
            unsafe {
                let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
                ::core::mem::transmute_copy(&::core::ptr::NonNull::new_unchecked(
                    &mut (*ptr).vtables.0 as *mut _ as _,
                ))
            }
        }
    }
    impl ::core::convert::From<&mut ReadOnlyCursorToStreamWrapper>
        for Windows::Win32::System::Com::IStream
    {
        fn from(implementation: &mut ReadOnlyCursorToStreamWrapper) -> Self {
            unsafe {
                let mut ptr = (implementation as *mut _ as *mut ::windows::core::RawPtr).sub(2 + 1)
                    as *mut ReadOnlyCursorToStreamWrapper_box;
                (*ptr).count.add_ref();
                ::core::mem::transmute_copy(&::core::ptr::NonNull::new_unchecked(
                    &mut (*ptr).vtables.0 as *mut _ as _,
                ))
            }
        }
    }
    impl ::windows::core::ToImpl<Windows::Win32::System::Com::IStream>
        for ReadOnlyCursorToStreamWrapper
    {
        unsafe fn to_impl(interface: &Windows::Win32::System::Com::IStream) -> &mut Self {
            let this: ::windows::core::RawPtr = core::mem::transmute_copy(interface);
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize)
                as *mut ReadOnlyCursorToStreamWrapper_box;
            &mut (*this).implementation
        }
    }
    impl ReadOnlyCursorToStreamWrapper {
        fn cast<ResultType: ::windows::core::Interface>(
            &self,
        ) -> ::windows::core::Result<ResultType> {
            unsafe {
                let boxed = (self as *const ReadOnlyCursorToStreamWrapper
                    as *mut ReadOnlyCursorToStreamWrapper
                    as *mut ::windows::core::RawPtr)
                    .sub(2 + 1)
                    as *mut ReadOnlyCursorToStreamWrapper_box;
                let mut result = None;
                (*boxed)
                    .QueryInterface(&ResultType::IID, &mut result as *mut _ as _)
                    .and_some(result)
            }
        }
    }
    impl ::core::convert::From<ReadOnlyCursorToStreamWrapper> for ::windows::core::IUnknown {
        fn from(implementation: ReadOnlyCursorToStreamWrapper) -> Self {
            let com = ReadOnlyCursorToStreamWrapper_box::new(implementation);
            unsafe {
                let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
                ::core::mem::transmute_copy(&::core::ptr::NonNull::new_unchecked(
                    &mut (*ptr).identity_vtable as *mut _ as _,
                ))
            }
        }
    }
    impl ::core::convert::From<ReadOnlyCursorToStreamWrapper> for ::windows::core::IInspectable {
        fn from(implementation: ReadOnlyCursorToStreamWrapper) -> Self {
            let com = ReadOnlyCursorToStreamWrapper_box::new(implementation);
            unsafe {
                let ptr = ::std::boxed::Box::into_raw(::std::boxed::Box::new(com));
                ::core::mem::transmute_copy(&::core::ptr::NonNull::new_unchecked(
                    &mut (*ptr).identity_vtable as *mut _ as _,
                ))
            }
        }
    }
    impl ::windows::core::Compose for ReadOnlyCursorToStreamWrapper {
        unsafe fn compose<'a>(
            implementation: Self,
        ) -> (
            ::windows::core::IInspectable,
            &'a mut core::option::Option<::windows::core::IInspectable>,
        ) {
            let inspectable: ::windows::core::IInspectable = implementation.into();
            let this = (&inspectable as *const _ as *mut ::windows::core::RawPtr).sub(1)
                as *mut ReadOnlyCursorToStreamWrapper_box;
            (inspectable, &mut (*this).base)
        }
    }
    #[repr(C)]
    struct ReadOnlyCursorToStreamWrapper_box {
        base: ::core::option::Option<::windows::core::IInspectable>,
        identity_vtable: *const ::windows::core::IInspectable_abi,
        vtables: (*const Windows::Win32::System::Com::IStream_abi,),
        implementation: ReadOnlyCursorToStreamWrapper,
        count: ::windows::core::WeakRefCount,
    }
    impl ReadOnlyCursorToStreamWrapper_box {
        const VTABLES: (Windows::Win32::System::Com::IStream_abi,) =
            (Windows::Win32::System::Com::IStream_abi(
                Self::QueryInterface_abi0,
                Self::AddRef_abi0,
                Self::Release_abi0,
                Self::abi1,
                Self::abi2,
                Self::abi3,
                Self::abi4,
                Self::abi5,
                Self::abi6,
                Self::abi7,
                Self::abi8,
                Self::abi9,
                Self::abi10,
                Self::abi11,
            ),);
        const IDENTITY_VTABLE: ::windows::core::IInspectable_abi =
            ::windows::core::IInspectable_abi(
                Self::identity_query_interface,
                Self::identity_add_ref,
                Self::identity_release,
                Self::GetIids,
                Self::GetRuntimeClassName,
                Self::GetTrustLevel,
            );
        const IID0: ::windows::core::GUID =
            <Windows::Win32::System::Com::IStream as ::windows::core::Interface>::IID;
        fn new(implementation: ReadOnlyCursorToStreamWrapper) -> Self {
            Self {
                base: ::core::option::Option::None,
                identity_vtable: &Self::IDENTITY_VTABLE,
                vtables: (&Self::VTABLES.0,),
                implementation,
                count: ::windows::core::WeakRefCount::new(),
            }
        }
        fn QueryInterface(
            &mut self,
            iid: &::windows::core::GUID,
            interface: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            unsafe {
                * interface = if iid == & < :: windows :: core :: IUnknown as :: windows :: core :: Interface > :: IID || iid == & < :: windows :: core :: IInspectable as :: windows :: core :: Interface > :: IID || iid == & < :: windows :: core :: IAgileObject as :: windows :: core :: Interface > :: IID { & mut self . identity_vtable as * mut _ as _ } else if iid == & Self :: IID0 { & mut self . vtables . 0 as * mut _ as _ } else if iid == & < Windows :: Win32 :: System :: Com :: ISequentialStream as :: windows :: core :: Interface > :: IID { & mut self . vtables . 0 as * mut _ as _ } else { :: core :: ptr :: null_mut () } ;
                if !(*interface).is_null() {
                    self.count.add_ref();
                    return ::windows::core::HRESULT(0);
                }
                *interface = self
                    .count
                    .query(iid, &mut self.identity_vtable as *mut _ as _);
                if (*interface).is_null() {
                    if let Some(base) = &self.base {
                        ::windows::core::Interface::query(base, iid, interface)
                    } else {
                        ::windows::core::HRESULT(0x8000_4002)
                    }
                } else {
                    ::windows::core::HRESULT(0)
                }
            }
        }
        fn AddRef(&mut self) -> u32 {
            self.count.add_ref()
        }
        fn Release(&mut self) -> u32 {
            let remaining = self.count.release();
            if remaining == 0 {
                unsafe {
                    ::std::boxed::Box::from_raw(self);
                }
            }
            remaining
        }
        unsafe extern "system" fn GetIids(
            _: ::windows::core::RawPtr,
            count: *mut u32,
            values: *mut *mut ::windows::core::GUID,
        ) -> ::windows::core::HRESULT {
            *count = 0;
            *values = ::core::ptr::null_mut();
            ::windows::core::HRESULT(0)
        }
        unsafe extern "system" fn GetRuntimeClassName(
            _: ::windows::core::RawPtr,
            value: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let h = ::windows::core::HSTRING::new();
            *value = ::core::mem::transmute(h);
            ::windows::core::HRESULT(0)
        }
        unsafe extern "system" fn GetTrustLevel(
            _: ::windows::core::RawPtr,
            value: *mut i32,
        ) -> ::windows::core::HRESULT {
            *value = 0;
            ::windows::core::HRESULT(0)
        }
        unsafe extern "system" fn identity_query_interface(
            this: ::windows::core::RawPtr,
            iid: &::windows::core::GUID,
            interface: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(1) as *mut Self;
            (*this).QueryInterface(iid, interface)
        }
        unsafe extern "system" fn identity_add_ref(this: ::windows::core::RawPtr) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).sub(1) as *mut Self;
            (*this).AddRef()
        }
        unsafe extern "system" fn identity_release(this: ::windows::core::RawPtr) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).sub(1) as *mut Self;
            (*this).Release()
        }
        unsafe extern "system" fn QueryInterface_abi0(
            this: ::windows::core::RawPtr,
            iid: &::windows::core::GUID,
            interface: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            (*this).QueryInterface(iid, interface)
        }
        unsafe extern "system" fn AddRef_abi0(this: ::windows::core::RawPtr) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            (*this).AddRef()
        }
        unsafe extern "system" fn Release_abi0(this: ::windows::core::RawPtr) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            (*this).Release()
        }
        unsafe extern "system" fn abi1(
            this: ::windows::core::RawPtr,
            pv: *mut ::core::ffi::c_void,
            cb: u32,
            pcbread: *mut u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            (*this)
                .implementation
                .Read(
                    ::core::mem::transmute_copy::<_, *mut ::core::ffi::c_void>(&pv),
                    ::core::mem::transmute_copy::<_, u32>(&cb),
                    ::core::mem::transmute_copy::<_, *mut u32>(&pcbread),
                )
                .into()
        }
        unsafe extern "system" fn abi2(
            this: ::windows::core::RawPtr,
            pv: *const ::core::ffi::c_void,
            cb: u32,
            pcbwritten: *mut u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            match (*this).implementation.Write(
                ::core::mem::transmute_copy::<_, *const ::core::ffi::c_void>(&pv),
                ::core::mem::transmute_copy::<_, u32>(&cb),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbwritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn abi3(
            this: ::windows::core::RawPtr,
            dlibmove: i64,
            dworigin: Windows::Win32::System::Com::STREAM_SEEK,
            plibnewposition: *mut u64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            match (*this).implementation.Seek(
                ::core::mem::transmute_copy::<_, i64>(&dlibmove),
                ::core::mem::transmute_copy::<_, Windows::Win32::System::Com::STREAM_SEEK>(
                    &dworigin,
                ),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *plibnewposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn abi4(
            this: ::windows::core::RawPtr,
            libnewsize: u64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            (*this)
                .implementation
                .SetSize(::core::mem::transmute_copy::<_, u64>(&libnewsize))
                .into()
        }
        unsafe extern "system" fn abi5(
            this: ::windows::core::RawPtr,
            pstm: ::windows::core::RawPtr,
            cb: u64,
            pcbread: *mut u64,
            pcbwritten: *mut u64,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            (*this)
                .implementation
                .CopyTo(
                    ::core::mem::transmute::<
                        _,
                        &::core::option::Option<Windows::Win32::System::Com::IStream>,
                    >(&pstm),
                    ::core::mem::transmute_copy::<_, u64>(&cb),
                    ::core::mem::transmute_copy::<_, *mut u64>(&pcbread),
                    ::core::mem::transmute_copy::<_, *mut u64>(&pcbwritten),
                )
                .into()
        }
        unsafe extern "system" fn abi6(
            this: ::windows::core::RawPtr,
            grfcommitflags: u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            (*this)
                .implementation
                .Commit(::core::mem::transmute_copy::<_, u32>(&grfcommitflags))
                .into()
        }
        unsafe extern "system" fn abi7(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            (*this).implementation.Revert().into()
        }
        unsafe extern "system" fn abi8(
            this: ::windows::core::RawPtr,
            liboffset: u64,
            cb: u64,
            dwlocktype: u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            (*this)
                .implementation
                .LockRegion(
                    ::core::mem::transmute_copy::<_, u64>(&liboffset),
                    ::core::mem::transmute_copy::<_, u64>(&cb),
                    ::core::mem::transmute_copy::<_, u32>(&dwlocktype),
                )
                .into()
        }
        unsafe extern "system" fn abi9(
            this: ::windows::core::RawPtr,
            liboffset: u64,
            cb: u64,
            dwlocktype: u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            (*this)
                .implementation
                .UnlockRegion(
                    ::core::mem::transmute_copy::<_, u64>(&liboffset),
                    ::core::mem::transmute_copy::<_, u64>(&cb),
                    ::core::mem::transmute_copy::<_, u32>(&dwlocktype),
                )
                .into()
        }
        unsafe extern "system" fn abi10(
            this: ::windows::core::RawPtr,
            pstatstg: *mut Windows::Win32::System::Com::STATSTG,
            grfstatflag: u32,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            (*this)
                .implementation
                .Stat(
                    ::core::mem::transmute_copy::<_, *mut Windows::Win32::System::Com::STATSTG>(
                        &pstatstg,
                    ),
                    ::core::mem::transmute_copy::<_, u32>(&grfstatflag),
                )
                .into()
        }
        unsafe extern "system" fn abi11(
            this: ::windows::core::RawPtr,
            ppstm: *mut ::windows::core::RawPtr,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).sub(2 + 0usize) as *mut Self;
            match (*this).implementation.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstm = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
    }
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
            let mut vec = ::alloc::vec::from_elem(0, slice.len());
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
        pub fn Read(
            &mut self,
            pv: *mut std::ffi::c_void,
            cb: u32,
            pcbread: *mut u32,
        ) -> Result<()> {
            let data = self.get_data_mut()?;
            let output = unsafe { std::slice::from_raw_parts_mut(pv as *mut u8, cb as usize) };
            let bytes_read = data.cursor.read(output).unwrap();
            unsafe { *pcbread = bytes_read as u32 };
            Ok(())
        }
        pub fn Write(&mut self, pv: *mut std::ffi::c_void, cb: u32) -> Result<u32> {
            E_NOTIMPL.ok()?;
            ::core::panicking::panic("not implemented")
        }
        pub fn Seek(&mut self, dlibmove: i64, dworigin: STREAM_SEEK) -> Result<u64> {
            let data = self.get_data_mut()?;
            let seek = match dworigin {
                STREAM_SEEK_CUR => SeekFrom::Current(dlibmove),
                STREAM_SEEK_END => SeekFrom::End(dlibmove),
                STREAM_SEEK_SET => SeekFrom::Start(dlibmove as u64),
            };
            let position = data.cursor.seek(seek).unwrap();
            Ok(position)
        }
        pub fn SetSize(&mut self, libnewsize: u64) -> Result<()> {
            E_NOTIMPL.ok()?;
            ::core::panicking::panic("not implemented")
        }
        pub fn CopyTo(
            &mut self,
            stream: &Option<IStream>,
            cb: u64,
            pcbread: *mut u64,
            pcbwritten: *mut u64,
        ) -> Result<()> {
            let data = self.get_data_mut()?;
            let mut bytes = ::alloc::vec::from_elem(0u8, cb as usize);
            let bytes_read = data.cursor.read(&mut bytes).unwrap();
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
            E_NOTIMPL.ok()?;
            ::core::panicking::panic("not implemented")
        }
        pub fn Revert(&mut self) -> Result<()> {
            E_NOTIMPL.ok()?;
            ::core::panicking::panic("not implemented")
        }
        pub fn LockRegion(&mut self, liboffset: u64, cb: u64, dwlocktype: u32) -> Result<()> {
            E_NOTIMPL.ok()?;
            ::core::panicking::panic("not implemented")
        }
        pub fn UnlockRegion(&mut self, liboffset: u64, cb: u64, dwlocktype: u32) -> Result<()> {
            E_NOTIMPL.ok()?;
            ::core::panicking::panic("not implemented")
        }
        pub fn Stat(&mut self, pstatstg: *mut STATSTG, grfstatflags: u32) -> Result<()> {
            ::core::panicking::panic("not implemented")
        }
        pub fn Clone(&mut self) -> Result<IStream> {
            E_NOTIMPL.ok()?;
            ::core::panicking::panic("not implemented")
        }
    }
}
