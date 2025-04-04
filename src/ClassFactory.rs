use std::os::raw::c_void;

use windows::Win32::{Foundation::{CLASS_E_NOAGGREGATION, E_FAIL}, System::Com::{IClassFactory, IClassFactory_Impl}};
use windows_core::{
    implement,
    Ref,
    GUID,
    Result,
    IUnknown,
};

use crate::profiler::AchtungBabyProfiler;

#[implement(IClassFactory)]
pub struct AchtungBabyClassFactory; 

impl IClassFactory_Impl for AchtungBabyClassFactory_Impl {
    fn CreateInstance(&self, punkouter: Ref<'_, IUnknown>, riid: *const GUID, ppvobject: *mut *mut c_void) -> windows_core::Result<()> {
        if !punkouter.is_null() {
            unsafe { *ppvobject = std::ptr::null_mut() };
            return Err(windows_core::Error::from_hresult(CLASS_E_NOAGGREGATION));
        }
        
        let profiler = AchtungBabyProfiler::new();
        if profiler.is_none() {
            return Err(windows_core::Error::from_hresult(E_FAIL));
        }

        todo!()
    }

    fn LockServer(&self, flock: windows_core::BOOL) -> windows_core::Result<()> {
        todo!()
    }
}