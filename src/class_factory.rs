use std::{os::raw::c_void, ptr};
use windows::Win32::{
    Foundation::{CLASS_E_NOAGGREGATION, E_FAIL, E_INVALIDARG, E_NOINTERFACE, E_POINTER},
    System::{
        Com::{IClassFactory, IClassFactory_Impl},
        Diagnostics::ClrProfiling::ICorProfilerCallback11,
    },
};
use windows_core::{implement, IUnknown, Interface, Ref, GUID};

use crate::profiler::AchtungBabyProfiler;

#[implement(IClassFactory)]
pub struct AchtungBabyClassFactory {}

impl IClassFactory_Impl for AchtungBabyClassFactory_Impl {
    fn CreateInstance(
        &self,
        punkouter: Ref<'_, IUnknown>,
        riid: *const GUID,
        ppvobject: *mut *mut c_void,
    ) -> windows_core::Result<()> {
        println!("[+] CreateInstance IID={:?}", riid);

        if punkouter.is_some() {
            return Err(CLASS_E_NOAGGREGATION.into());
        }
        if ppvobject.is_null() {
            return Err(E_POINTER.into());
        }

        unsafe { *ppvobject = ptr::null_mut() };

        if riid.is_null() {
            return Err(E_INVALIDARG.into());
        }

        let riid = unsafe { *riid };

        if !AchtungBabyProfiler::is_available_interface(riid) {
            return Err(E_NOINTERFACE.into());
        }

        let profiler = ICorProfilerCallback11::from(AchtungBabyProfiler::new());

        if profiler.as_raw().is_null() {
            return Err(windows_core::Error::from_hresult(E_FAIL));
        }

        unsafe {
            let result = profiler.query(&riid, ppvobject);
            match result.is_ok() {
                true => println!(
                    "[+] AchtungBabyProfiler::query succeeded: {:?}",
                    result.message()
                ),
                false => println!(
                    "[-] AchtungBabyProfiler::query failed: {:?}",
                    result.message()
                ),
            }

            result.ok()
        }
    }

    fn LockServer(&self, _flock: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }
}
