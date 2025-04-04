use std::os::raw::c_void;

use windows::Win32::{Foundation::{CLASS_E_NOAGGREGATION, E_FAIL, S_OK}, System::{Com::{IClassFactory, IClassFactory_Impl}, Diagnostics::ClrProfiling::{ICorProfilerCallback, ICorProfilerCallback2}}, UI::WindowsAndMessaging::MessageBoxW};
use windows_core::{
    implement, w, IUnknown, Interface, Ref, GUID, HSTRING
};

use crate::profiler::{ AchtungBabyProfiler, IAchtungBabyProfiler };

#[implement(IClassFactory)]
pub struct AchtungBabyClassFactory; 

impl AchtungBabyClassFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl IClassFactory_Impl for AchtungBabyClassFactory_Impl {
    fn CreateInstance(&self, punkouter: Ref<'_, IUnknown>, riid: *const GUID, ppvobject: *mut *mut c_void) -> windows_core::Result<()> {
        if !punkouter.is_null() {
            unsafe { *ppvobject = std::ptr::null_mut() };
            return Err(windows_core::Error::from_hresult(CLASS_E_NOAGGREGATION));
        }
        
        let profiler: IAchtungBabyProfiler = AchtungBabyProfiler::new().into();
        if profiler.as_raw().is_null() {
            return Err(windows_core::Error::from_hresult(E_FAIL));
        }

        unsafe { 
            profiler.query(riid, ppvobject).ok()
        }
    }

    fn LockServer(&self, flock: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }
}