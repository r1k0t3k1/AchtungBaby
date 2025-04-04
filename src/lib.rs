use windows::Win32::Foundation::{
    E_INVALIDARG, E_POINTER, HMODULE, HWND, S_OK
};
use windows::Win32::System::SystemServices::{
    DLL_PROCESS_ATTACH,
    DLL_PROCESS_DETACH,
};
//use windows::Win32::System::Com::*;
use windows::core::{HRESULT, GUID, w};
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use core::ffi::c_void;

mod profiler;
mod ClassFactory;

// DLLロード時は特に何もしなくていい
#[no_mangle]
extern "system" fn DllMain(dll_module: HMODULE, call_reason: u32, _lp_reserved: *mut c_void) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => {

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
    S_OK
}

// 常にDLLアンロードを許可
#[no_mangle]
extern "stdcall" fn DllCanUnloadNow() -> HRESULT {
    S_OK
}