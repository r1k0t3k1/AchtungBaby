use profiler::{AchtungBabyProfiler, CLSID_PROFILER};
use util::Logger;
use windows::Win32::Foundation::{
    CLASS_E_CLASSNOTAVAILABLE, E_INVALIDARG, E_POINTER, HMODULE, HWND, S_OK
};
use windows::Win32::System::Com::{IClassFactory, IClassFactory_Impl};
use windows::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH,
    DLL_PROCESS_DETACH,
};
//use windows::Win32::System::Com::*;
use windows::core::{HRESULT, GUID, w};
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use windows_core::{ComObjectInner, Interface, HSTRING};
use class_factory::{AchtungBabyClassFactory, AchtungBabyClassFactory_Impl};
use core::ffi::c_void;

mod profiler;
mod class_factory;
mod util;

// DLLロード時は特に何もしなくていい
#[no_mangle]
extern "system" fn DllMain(dll_module: HMODULE, call_reason: u32, _lp_reserved: *mut c_void) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            println!("logging");
        }
        DLL_PROCESS_DETACH => {

        }
        _ => (),
    }

    return true;
}

// DLL読み込みテスト用関数
// rundll32.exe AchtungBaby.dll,LoadTest
#[no_mangle]
extern "stdcall" fn LoadTest() -> HRESULT {
    unsafe {
        MessageBoxW(
            None,
            w!("test"),
            w!("title"),
            Default::default(),
        );
    }
    S_OK
}


// riidで要求されたインターフェイスへのポインタを返却する
#[no_mangle]
extern "stdcall" fn DllGetClassObject(
    rclsid: *const GUID,
    riid: *const GUID,
    ppv: *mut c_void,
) -> HRESULT {      
    if ppv.is_null() {
        return E_POINTER;
    }
    if rclsid.is_null() || riid.is_null() {
        return E_INVALIDARG
    }

    let clsid = unsafe { *rclsid };
    let iid: GUID = unsafe { *riid };
    if clsid != CLSID_PROFILER || iid != IClassFactory::IID {
        return CLASS_E_CLASSNOTAVAILABLE;
    }

    println!("DllGetClassObject: CLSID={:?}, IID={:?}", clsid, iid);
    let factory: IClassFactory = IClassFactory::from(AchtungBabyClassFactory {});
    unsafe { 
        let result = factory.query(riid,  ppv as _);
        match result.is_ok() {
            true => println!("IClassFacotry query succeeded: {:?}", result.message()),
            false => println!("IClassFacotry query failed: {:?}", result.message()),
        }
        result
    }
}

// 常にDLLアンロードを許可
#[no_mangle]
extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    S_OK
}