use profiler::CLSID_PROFILER;
use windows::Win32::Foundation::{
    CLASS_E_CLASSNOTAVAILABLE, E_INVALIDARG, E_POINTER, S_OK
};
use windows::Win32::System::Com::IClassFactory;
use windows::core::{ HRESULT, GUID };
use windows_core::Interface;
use class_factory::AchtungBabyClassFactory;
use core::ffi::c_void;

mod profiler;
mod class_factory;
mod util;

// DLLロード時は特に何もしなくていいのでDllMainは定義し無くてもいい
//#[no_mangle]
//extern "system" fn DllMain(_dll_module: HMODULE, call_reason: u32, _lp_reserved: *mut c_void) -> bool {
//    match call_reason {
//        DLL_PROCESS_ATTACH => {
//            println!("[+] DLL Attached");
//        }
//        DLL_PROCESS_DETACH => {
//
//        }
//        _ => (),
//    }
//
//    return true;
//}

// 実質のエントリポイント
// riidで要求されたインターフェイスへのポインタを返却する
// 実装例： https://learn.microsoft.com/ja-jp/windows/win32/api/combaseapi/nf-combaseapi-dllgetclassobject
#[no_mangle]
extern "system" fn DllGetClassObject(
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

    println!("[+] DllGetClassObject: CLSID={:?}, IID={:?}", clsid, iid);

    let factory: IClassFactory = IClassFactory::from(AchtungBabyClassFactory {});
    unsafe { 
        let result = factory.query(riid,  ppv as *mut *mut c_void);
        match result.is_ok() {
            true => println!("[+] IClassFacotry::query succeeded: {:?}", result.message()),
            false => println!("[-] IClassFacotry::query failed: {:?}", result.message()),
        }
        result
    }
}

// 常にDLLアンロードを許可
#[no_mangle]
extern "system" fn DllCanUnloadNow() -> HRESULT {
    S_OK
}