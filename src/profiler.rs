use windows::core::*;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::System::Diagnostics::ClrProfiling::{
    ICorProfilerCallback,
    ICorProfilerCallback_Impl, ICorProfilerInfo, COR_PRF_MONITOR_ASSEMBLY_LOADS,
};
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use core::ffi::c_void;
use std::cell::RefCell;
use std::mem;

// {5c8e9579-53b9-5a69-6a75-2d232518df35}
const CLSID_PROFILER: GUID = GUID::from_values(
    0x5c8e9579,
    0x53b9,
    0x5a69,
    [0x6a,0x75,0x2d,0x23,0x25,0x18,0xdf,0x35],
);

// COM interface 実装サンプル
// https://docs.rs/windows-core/latest/windows_core/attr.implement.html
#[interface("5c8e9579-53b9-5a69-6a75-2d232518df35")]
unsafe trait IAchtungBabyProfiler: IUnknown {
    pub fn set(&self, value: ICorProfilerInfo); 
}

#[implement(IAchtungBabyProfiler,ICorProfilerCallback)]
pub struct AchtungBabyProfiler {
    profiler_info: RefCell<Option<ICorProfilerInfo>>,
}

impl AchtungBabyProfiler {
    pub fn new() -> Self {
        Self {
            profiler_info: RefCell::new(None),
        }
    }

    pub fn is_none(&self) -> bool {
        self.profiler_info.borrow().is_none()
    }
}

impl IAchtungBabyProfiler_Impl for AchtungBabyProfiler_Impl {
    unsafe fn set(&self,value:ICorProfilerInfo) {
        let mut info = self.profiler_info.borrow_mut();
        *info = Some(value);
    }
}

impl ICorProfilerCallback_Impl for AchtungBabyProfiler_Impl {
    fn Initialize(&self, picorprofilerinfounk: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()> {
        // 引数として渡ってくるICorProfilerInfoをIUnknown経由で取得する
        let profiler_info = picorprofilerinfounk.as_ref().unwrap();
        let interface = std::ptr::null_mut();
        unsafe {
            let result = profiler_info.query(&ICorProfilerInfo::IID as *const GUID, interface);
            if result.is_err() {
                return Err(windows_core::Error::from_hresult(E_FAIL));
            }
            let queried_profiler_info = ICorProfilerInfo::from_raw(*interface);
            self.set(queried_profiler_info);
            let _ = self.profiler_info.borrow().as_ref().unwrap().SetEventMask(COR_PRF_MONITOR_ASSEMBLY_LOADS.0 as u32);
        };
        Ok(())
    }

    fn Shutdown(&self) -> windows_core::Result<()> {
        // ICorProfilerInfoの解放
        *self.profiler_info.borrow_mut() = None;
        Ok(())
    }

    fn AppDomainCreationStarted(&self, appdomainid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn AppDomainShutdownStarted(&self, appdomainid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn AssemblyLoadStarted(&self, assemblyid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe {
            MessageBoxW(
                None,
                w!("test"),
                w!("AssemblyLoadFinished"),
                Default::default(),
            );
        }
        Ok(())
    }

    fn AssemblyUnloadStarted(&self, assemblyid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn ModuleLoadStarted(&self, moduleid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn ModuleUnloadStarted(&self, moduleid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ClassLoadStarted(&self, classid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ClassLoadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn ClassUnloadStarted(&self, classid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ClassUnloadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        Ok(())
    }

    fn FunctionUnloadStarted(&self, functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn JITCompilationStarted(&self, functionid: usize, fissafetoblock: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn JITCompilationFinished(&self, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn JITCachedFunctionSearchStarted(&self, functionid: usize) -> windows_core::Result<windows_core::BOOL> {
        Ok(BOOL(1))
    }

    fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_JIT_CACHE) -> windows_core::Result<()> {
        Ok(())
    }

    fn JITFunctionPitched(&self, functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn JITInlining(&self, callerid: usize, calleeid: usize) -> windows_core::Result<windows_core::BOOL> {
        Ok(BOOL(1))
    }

    fn ThreadCreated(&self, threadid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ThreadDestroyed(&self, threadid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingClientInvocationStarted(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingClientSendingMessage(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingClientReceivingReply(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingClientInvocationFinished(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingServerReceivingMessage(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingServerInvocationStarted(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingServerInvocationReturned(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn RemotingServerSendingReply(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn UnmanagedToManagedTransition(&self, functionid: usize, reason: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_TRANSITION_REASON) -> windows_core::Result<()> {
        Ok(())
    }

    fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_TRANSITION_REASON) -> windows_core::Result<()> {
        Ok(())
    }

    fn RuntimeSuspendStarted(&self, suspendreason: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_SUSPEND_REASON) -> windows_core::Result<()> {
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

    fn RuntimeThreadSuspended(&self, threadid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn RuntimeThreadResumed(&self, threadid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::Result<()> {
        Ok(())
    }

    fn ObjectAllocated(&self, objectid: usize, classid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *const usize, cobjects: *const u32) -> windows_core::Result<()> {
        Ok(())
    }

    fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *const usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn RootReferences(&self, crootrefs: u32, rootrefids: *const usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionThrown(&self, thrownobjectid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionSearchFunctionLeave(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionSearchFilterEnter(&self, functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionSearchFilterLeave(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionSearchCatcherFound(&self, functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionOSHandlerEnter(&self, __unused: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionOSHandlerLeave(&self, __unused: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionUnwindFunctionLeave(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionUnwindFinallyLeave(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionCatcherLeave(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void, cslots: u32) -> windows_core::Result<()> {
        Ok(())
    }

    fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionCLRCatcherFound(&self) -> windows_core::Result<()> {
        Ok(())
    }

    fn ExceptionCLRCatcherExecute(&self) -> windows_core::Result<()> {
        Ok(())
    }
}