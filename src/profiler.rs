use std::sync::OnceLock;

use windows::core::*;
use windows::Win32::Foundation::{E_UNEXPECTED, MAX_PATH};
use windows::Win32::System::Diagnostics::ClrProfiling::{
    ICorProfilerCallback, ICorProfilerCallback2, ICorProfilerCallback2_Impl, ICorProfilerCallback3, ICorProfilerCallback3_Impl, ICorProfilerCallback4, ICorProfilerCallback4_Impl, ICorProfilerCallback5, ICorProfilerCallback5_Impl, ICorProfilerCallback_Impl, ICorProfilerInfo3, COR_PRF_CODE_INFO, COR_PRF_MONITOR_ASSEMBLY_LOADS, COR_PRF_MONITOR_JIT_COMPILATION, COR_PRF_USE_PROFILE_IMAGES
};
use windows::Win32::System::Diagnostics::Debug::MAX_SYM_NAME;
use windows::Win32::System::WinRT::Metadata::{IMetaDataImport2, IMAGE_COR_ILMETHOD, IMAGE_COR_ILMETHOD_FAT, IMAGE_COR_ILMETHOD_TINY};

use crate::util::Logger;

// {5c8e9579-53b9-5a69-6a75-2d232518df35}
pub const CLSID_PROFILER: GUID = GUID::from_values(
    0x5c8e9579,
    0x53b9,
    0x5a69,
    [0x6a,0x75,0x2d,0x23,0x25,0x18,0xdf,0x35],
);

// COM interface 実装サンプル
// https://docs.rs/windows-core/latest/windows_core/attr.implement.html
// ICorProfilerCallback2を実装していないとqueriInterfaceの際に「インターフェースがサポートされていません。」エラーになる
// ICorProfilerCallbackのIID:  176FBED1-A55C-4796-98CA-A9DA0EF883E7
// ICorProfilerCallback2のIID: 8A8CC829-CCF2-49fe-BBAE-0F022228071A
#[implement(
    ICorProfilerCallback5,
    ICorProfilerCallback4,
    ICorProfilerCallback3,
    ICorProfilerCallback2,
    ICorProfilerCallback,
)]
pub struct AchtungBabyProfiler {
    profiler_info: OnceLock<ICorProfilerInfo3>,
}

impl AchtungBabyProfiler {
    pub fn new() -> Self {
        Self { profiler_info: OnceLock::new() }
    }

    fn set_profiler_info(&self, value: ICorProfilerInfo3) -> windows_core::Result<()> {
        self.profiler_info.set(value)
            .map_err(|_| windows_core::Error::new(
                E_UNEXPECTED,
                "Assignment of ICorProfilerInfo to a global variable failed."
            )
        )
    }

    fn get_profiler_info(&self) -> Option<&ICorProfilerInfo3> {
        self.profiler_info.get()
    }
    fn get_function_info(&self, functionid: usize) -> windows_core::Result<(String, u32)> {
        let mut ppimport:Option<IUnknown> = None; 
        let mut ptoken = 0_u32;  // GetMethodPropsに渡すようのメタデータトークン
        unsafe {
            // functionidで指定した関数のメタデータとIMetaDataImportインターフェースを取得する
            // https://learn.microsoft.com/ja-jp/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-gettokenandmetadatafromfunction-method
            self.get_profiler_info().unwrap().GetTokenAndMetaDataFromFunction(
                functionid,
                &IMetaDataImport2::IID,
                &mut ppimport,
                &mut ptoken
            )?;
        }

        // IMetaDataImport2
        // https://learn.microsoft.com/ja-jp/windows/win32/api/rometadataapi/nn-rometadataapi-imetadataimport2
        // GetTokenAndMetaDataFromFunctionで取得できるのはIUnknownなのでcast(QueryInterface)を使用してIMetaDataImport2に変換する
        let pimport = ppimport.unwrap().cast::<IMetaDataImport2>()?;

        let mut pclass = 0_u32;
        let mut szmethod: [u16; MAX_SYM_NAME as usize] = [0; MAX_SYM_NAME as usize];
        let mut pchmethod = 0_u32;
        let mut pdwattr = 0_u32;
        let ppvsigblob = 0_u8;
        let mut pcbsigblob = 0_u32;
        let mut pulcoderva = 0_u32;
        let mut pdwimplflags = 0_u32;

        unsafe {
            // メソッドのメタデータ取得
            // https://learn.microsoft.com/ja-jp/dotnet/framework/unmanaged-api/metadata/imetadataimport-getmethodprops-method
            pimport.GetMethodProps(
                ptoken,
                &mut pclass, 
                Some(&mut szmethod), 
                &mut pchmethod, 
                &mut pdwattr, 
                ppvsigblob as *mut *mut u8,
                &mut pcbsigblob, 
                &mut pulcoderva, 
                &mut pdwimplflags
            )?;
        }

        // 末尾のNULL文字削除
        let filterd: Vec<u16> = szmethod.into_iter().filter(|x| *x != 0).collect();
        let method_name = HSTRING::from_wide(&filterd);

        let mut sztypedef: [u16; MAX_SYM_NAME as usize] = [0; MAX_SYM_NAME as usize];
        let mut pchtypedef = 0_u32;
        let mut pdwtypedefflags = 0_u32;
        let mut ptkextends = 0_u32;
 
       unsafe {
            // メソッドが取得されているクラスのメタデータ取得
            // https://learn.microsoft.com/ja-jp/dotnet/framework/unmanaged-api/metadata/imetadataimport-gettypedefprops-method
            pimport.GetTypeDefProps(
                pclass,
                Some(&mut sztypedef),
                &mut pchtypedef,
                &mut pdwtypedefflags,
                &mut ptkextends
            )?;
        }

        // 末尾のNULL文字削除
        let class_filterd: Vec<u16> = sztypedef.into_iter().filter(|x| *x != 0).collect();
        let class_name = HSTRING::from_wide(&class_filterd);

        Ok((format!("{}.{}", class_name, method_name), ptoken))
    }
}

impl ICorProfilerCallback_Impl for AchtungBabyProfiler_Impl {
    fn Initialize(&self, picorprofilerinfounk: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()> {
        println!("[+] ICorProfilerCallback::Initialize");
        // 引数として渡ってくるICorProfilerInfoをIUnknown経由で取得する
        let profiler_info = picorprofilerinfounk.unwrap();
        println!("[+] ICorProfilerInfo: {:?}", profiler_info);

        self.set_profiler_info(profiler_info.cast::<ICorProfilerInfo3>()?)?;

        println!("[+] get ICorProfilerInfo");
        unsafe { self.get_profiler_info().unwrap().SetEventMask(
            COR_PRF_MONITOR_ASSEMBLY_LOADS.0 as u32 | // アセンブリの読み込み通知を購読
            COR_PRF_MONITOR_JIT_COMPILATION.0 as u32 |         // JITの開始通知を購読
            COR_PRF_USE_PROFILE_IMAGES.0 as u32                // NGENにより予めJITコンパイルされたライブラリにおいてもJITコンパイルさせる
        )? };

        println!("[+] ICorProfilerInfo::SetEventMask");
        Ok(())
    }

    fn Shutdown(&self) -> windows_core::Result<()> {
        ("shutdown called");
        // ICorProfilerInfoの解放
        //*self.profiler_info.borrow_mut() = None;
        Ok(())
    }

    fn JITCompilationStarted(&self, functionid: usize, _fissafetoblock: windows_core::BOOL) -> windows_core::Result<()> {
        let (method_name, ptoken) = self.get_function_info(functionid)?;
        let mut ptoken = ptoken;

        if method_name == "System.Management.Automation.AmsiUtils.ScanContent" {
            let mut pclassid = 0_usize;
            let mut pmoduleid = 0_usize;

            unsafe {
                // メソッドが定義されているモジュールのmoduleidが欲しい
                self.get_profiler_info().unwrap().GetFunctionInfo(
                    functionid,
                    &mut pclassid,
                    &mut pmoduleid,
                    &mut ptoken
                )?;
            }

            let mut ppmethodheader = std::ptr::null_mut();
            let mut pcbmethodsize = 0_u32;

            unsafe {
                self.get_profiler_info().unwrap().GetILFunctionBody(
                    pmoduleid, 
                    ptoken, 
                    &mut ppmethodheader, 
                    &mut pcbmethodsize
                )?;
            }

            println!("ptoken: {}", ptoken);
            let il_bytes = unsafe { std::slice::from_raw_parts(ppmethodheader, pcbmethodsize as usize) };
            let il_method = unsafe { *(il_bytes.as_ptr() as *const IMAGE_COR_ILMETHOD) };
            println!("FAT: {:?}", unsafe { il_method.Fat } );
            println!("TINY: {:?}", unsafe { il_method.Tiny } );

            let fat_header_size = size_of::<IMAGE_COR_ILMETHOD_FAT>();
            let mut cloned_header = il_method.clone();

            // https://learn.microsoft.com/en-us/dotnet/api/system.reflection.emit.opcodes?view=net-9.0
            let new_il:[u8;2] = [
                0x16,  // push 0 to stack top
                0x2a, // ret
            ];

            cloned_header.Fat.CodeSize = new_il.len() as u32;
            cloned_header.Tiny = IMAGE_COR_ILMETHOD_TINY::default();
            cloned_header.Tiny.Flags_CodeSize = 0b001010_u8;

            let total_size = fat_header_size + new_il.len();

            let method_alloc = unsafe { self.get_profiler_info().unwrap().GetILFunctionBodyAllocator(pmoduleid)? };
            let allocated = unsafe { method_alloc.Alloc(total_size as u32) as *mut u8 };

            //let a1: [u8; size_of::<IMAGE_COR_ILMETHOD_FAT>()] = unsafe { std::mem::transmute(cloned_header.Fat) };
            let a1: [u8; size_of::<IMAGE_COR_ILMETHOD_TINY>()] = unsafe { std::mem::transmute(cloned_header.Tiny) };
            println!("a1: {:?}", a1);
            //unsafe { std::ptr::copy_nonoverlapping(a1.as_ptr(), allocated, fat_header_size) } ;
            unsafe { std::ptr::copy_nonoverlapping(a1.as_ptr(), allocated, size_of::<IMAGE_COR_ILMETHOD_TINY>()) } ;
            unsafe { std::ptr::copy_nonoverlapping(new_il.as_ptr(), allocated.add(a1.len()), new_il.len()) };

            unsafe{ 
                let r = self.get_profiler_info().unwrap()
                    .SetILFunctionBody(pmoduleid, ptoken, allocated);
                if r.is_err() {
                    println!("{:?}", r);
                }
            };
        }

        Ok(())
    }

    fn JITCompilationFinished(&self, functionid: usize, _hrstatus: windows_core::HRESULT, _fissafetoblock: windows_core::BOOL) -> windows_core::Result<()> {
        let (method_name, _ptoken) = self.get_function_info(functionid)?;
        if method_name != "System.Management.Automation.AmsiUtils.ScanContent" {
            return Ok(());
        }

        let mut pccodeinfos = 0_u32;
        let mut codeinfos: [COR_PRF_CODE_INFO;0] = [];

        unsafe {
            self.get_profiler_info().unwrap().GetCodeInfo2(
                functionid, 
                &mut pccodeinfos,
                &mut codeinfos,
            )?;
        }

        let mut codeinfos: [COR_PRF_CODE_INFO;1] = [Default::default()];

        unsafe {
            self.get_profiler_info().unwrap().GetCodeInfo2(
                functionid, 
                &mut pccodeinfos,
                &mut codeinfos,
            )?;
        }

        let bytes = unsafe { std::slice::from_raw_parts(codeinfos[0].startAddress as *const u8, codeinfos[0].size) };
        Logger::show_disasm(bytes);
        Ok(())
    }

    fn AssemblyLoadFinished(&self, assemblyid: usize, _hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        let pcchname = 0_u32;
        let mut szname: [u16; MAX_PATH as usize] = [0; MAX_PATH as usize];
        let mut pappdomainid = 0_usize;
        let mut pmoduleid = 0_usize;

        unsafe {
            // assemblyidからロードしたアセンブリの情報を取得する
            // https://learn.microsoft.com/ja-jp/dotnet/framework/unmanaged-api/profiling/icorprofilerinfo-getassemblyinfo-method
            self.get_profiler_info().unwrap().GetAssemblyInfo(
                assemblyid,
                pcchname as *mut u32, 
                &mut szname, 
                &mut pappdomainid, 
                &mut pmoduleid
            )?;
        }

        let assembly_name = &HSTRING::from_wide(&szname);

        let _debug_info = format!("assemblyid: {:x}\nszname: {}\npappdomainid: {:x},\npmoduleid: {:x}",
            assemblyid,
            assembly_name,
            pappdomainid,
            pmoduleid, 
        );
        //println!("{}", debug_info);

        Ok(())
    }

    fn AppDomainCreationStarted(&self, _appdomainid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn AppDomainCreationFinished(&self, _appdomainid: usize, _hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn AppDomainShutdownStarted(&self, _appdomainid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn AppDomainShutdownFinished(&self, _appdomainid: usize, _hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn AssemblyLoadStarted(&self, _assemblyid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn AssemblyUnloadStarted(&self, _assemblyid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn AssemblyUnloadFinished(&self, _assemblyid: usize, _hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn ModuleLoadStarted(&self, _moduleid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ModuleLoadFinished(&self, _moduleid: usize, _hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn ModuleUnloadStarted(&self, _moduleid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ModuleUnloadFinished(&self, _moduleid: usize, _hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn ModuleAttachedToAssembly(&self, _moduleid: usize, _assemblyid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ClassLoadStarted(&self, _classid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ClassLoadFinished(&self, _classid: usize, _hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn ClassUnloadStarted(&self, _classid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ClassUnloadFinished(&self, _classid: usize, _hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn FunctionUnloadStarted(&self, _functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn JITCachedFunctionSearchStarted(&self, _functionid: usize) -> windows_core::Result<windows_core::BOOL> {
        Ok(true.into())
    }

    fn JITCachedFunctionSearchFinished(&self, _functionid: usize, _result: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_JIT_CACHE) -> windows_core::Result<()> {
        Ok(())
    }

    fn JITFunctionPitched(&self, _functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn JITInlining(&self, _callerid: usize, _calleeid: usize) -> windows_core::Result<windows_core::BOOL> {
        Ok(true.into())
    }

    fn ThreadCreated(&self, _threadid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ThreadDestroyed(&self, _threadid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ThreadAssignedToOSThread(&self, _managed_threadid: usize, _os_threadid: u32) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingClientInvocationStarted(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingClientSendingMessage(&self, _pcookie: *const windows_core::GUID, _fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingClientReceivingReply(&self, _pcookie: *const windows_core::GUID, _fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingClientInvocationFinished(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingServerReceivingMessage(&self, _pcookie: *const windows_core::GUID, _fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingServerInvocationStarted(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingServerInvocationReturned(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingServerSendingReply(&self, _pcookie: *const windows_core::GUID, _fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn UnmanagedToManagedTransition(&self, _functionid: usize, __reason: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_TRANSITION_REASON) -> windows_core::Result<()> {
        Ok(())
    }

    fn ManagedToUnmanagedTransition(&self, _functionid: usize, __reason: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_TRANSITION_REASON) -> windows_core::Result<()> {
        Ok(())
    }

    fn RuntimeSuspendStarted(&self, _suspend_reason: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_SUSPEND_REASON) -> windows_core::Result<()> {
        Ok(())
    }

    fn RuntimeSuspendFinished(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RuntimeSuspendAborted(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RuntimeResumeStarted(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RuntimeResumeFinished(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RuntimeThreadSuspended(&self, _threadid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn RuntimeThreadResumed(&self, _threadid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn MovedReferences(&self, _cmoved_objectidranges: u32, _old_objectidrangestart: *const usize, _new_objectidrangestart: *const usize, _c_objectidrangelength: *const u32) -> windows_core::Result<()> {
        Ok(())
    }

    fn ObjectAllocated(&self, __objectid: usize, _classid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ObjectsAllocatedByClass(&self, _cclasscount: u32, _classids: *const usize, _cobjects: *const u32) -> windows_core::Result<()> {
        Ok(())
    }

    fn ObjectReferences(&self, __objectid: usize, _classid: usize, _cobjectrefs: u32, _objectrefids: *const usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn RootReferences(&self, _crootrefs: u32, _rootrefids: *const usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionThrown(&self, _thrown_objectid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionSearchFunctionEnter(&self, _functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionSearchFunctionLeave(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionSearchFilterEnter(&self, _functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionSearchFilterLeave(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionSearchCatcherFound(&self, _functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionOSHandlerEnter(&self, __unused: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionOSHandlerLeave(&self, __unused: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionUnwindFunctionEnter(&self, _functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionUnwindFunctionLeave(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionUnwindFinallyEnter(&self, _functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionUnwindFinallyLeave(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionCatcherEnter(&self, _functionid: usize, _objectid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionCatcherLeave(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn COMClassicVTableCreated(&self, _wrapped_classid: usize, _implementediid: *const windows_core::GUID, _pvtable: *const core::ffi::c_void, _cslots: u32) -> windows_core::Result<()> {
        Ok(())
    }

    fn COMClassicVTableDestroyed(&self, _wrapped_classid: usize, _implementediid: *const windows_core::GUID, _pvtable: *const core::ffi::c_void) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionCLRCatcherFound(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionCLRCatcherExecute(&self) -> windows_core::Result<()> {
        Ok(())
    }
}


impl ICorProfilerCallback2_Impl for AchtungBabyProfiler_Impl {
    fn ThreadNameChanged(&self, _threadid: usize, _cchname: u32, _name: &windows_core::PCWSTR) -> windows_core::Result<()> {
        Ok(())
    }

    fn GarbageCollectionStarted(&self, _cgenerations: i32, _generationcollected: *const windows_core::BOOL, _reason: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_GC_REASON) -> windows_core::Result<()> {
        Ok(())
    }

    fn SurvivingReferences(&self, _csurvivingobjectidranges: u32, _objectidrangestart: *const usize, _cobjectidrangelength: *const u32) -> windows_core::Result<()> {
        Ok(())
    }

    fn GarbageCollectionFinished(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn FinalizeableObjectQueued(&self, _finalizerflags: u32, _objectid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn RootReferences2(&self, _crootrefs: u32, _rootrefids: *const usize, _rootkinds: *const windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_GC_ROOT_KIND, _rootflags: *const windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_GC_ROOT_FLAGS, _rootids: *const usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn HandleCreated(&self, _handleid: usize, _initialobjectid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn HandleDestroyed(&self, _handleid: usize) -> windows_core::Result<()> {
        Ok(())
    }
}

impl ICorProfilerCallback3_Impl for AchtungBabyProfiler_Impl {
    fn InitializeForAttach(&self, _pcorprofilerinfounk: windows_core::Ref<'_, windows_core::IUnknown>, _pvclientdata: *const core::ffi::c_void, _cbclientdata: u32) -> windows_core::Result<()> {
        Ok(())
    }

    fn ProfilerAttachComplete(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn ProfilerDetachSucceeded(&self) -> windows_core::Result<()> {
        Ok(())
    }
}

impl ICorProfilerCallback4_Impl for AchtungBabyProfiler_Impl {
    fn ReJITCompilationStarted(&self, _functionid: usize, _rejitid: usize, _fissafetoblock: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn GetReJITParameters(&self, _moduleid: usize, _methodid: u32, _pfunctioncontrol: windows_core::Ref<'_, windows::Win32::System::Diagnostics::ClrProfiling::ICorProfilerFunctionControl>) -> windows_core::Result<()> {
        Ok(())
    }

    fn ReJITCompilationFinished(&self, _functionid: usize, _rejitid: usize, _hrstatus: windows_core::HRESULT, _fissafetoblock: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn ReJITError(&self, _moduleid: usize, _methodid: u32, _functionid: usize, _hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn MovedReferences2(&self, _cmovedobjectidranges: u32, _oldobjectidrangestart: *const usize, _newobjectidrangestart: *const usize, _cobjectidrangelength: *const usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn SurvivingReferences2(&self, _csurvivingobjectidranges: u32, _objectidrangestart: *const usize, _cobjectidrangelength: *const usize) -> windows_core::Result<()> {
        Ok(())
    }
}
impl ICorProfilerCallback5_Impl for AchtungBabyProfiler_Impl {
    fn ConditionalWeakTableElementReferences(&self, _crootrefs: u32, _keyrefids: *const usize, _valuerefids: *const usize, _rootids: *const usize) -> windows_core::Result<()> {
        Ok(())
    }
}