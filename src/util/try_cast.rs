use bindings::Windows::Win32::Foundation::E_NOINTERFACE;
use windows::{Abi, Interface};

pub trait TryCast {
    fn try_cast<T: Interface + Abi>(&self) -> windows::Result<Option<T>>;
}

impl<Base: Interface> TryCast for Base {
    fn try_cast<T: Interface + Abi>(&self) -> windows::Result<Option<T>> {
        let mut result = None;
        let code = unsafe { self.query(&T::IID, &mut result as *mut _ as _) };
        if code == E_NOINTERFACE {
            Ok(None)
        } else if code.is_ok() {
            Ok(result)
        } else {
            code.ok()?;
            unreachable!()
        }
    }
}