use windows::core::*;
use windows::Win32::Foundation::E_FAIL;
use windows::Win32::System::Diagnostics::ClrProfiling::{
    ICorProfilerCallback,ICorProfilerCallback2, ICorProfilerCallback_Impl, ICorProfilerCallback2_Impl,ICorProfilerInfo, COR_PRF_MONITOR_ASSEMBLY_LOADS
};
use windows::Win32::UI::WindowsAndMessaging::MessageBoxW;
use std::cell::RefCell;

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
// ICorProfilerCallback2のIID: 8A8CC829-CCF2-49fe-BBAE-0F022228071A
#[implement(IAchtungBabyProfiler, ICorProfilerCallback, ICorProfilerCallback2)]
pub struct AchtungBabyProfiler {
    profiler_info: RefCell<Option<ICorProfilerInfo>>,
}

#[interface("5c8e9579-53b9-5a69-6a75-2d232518df35")]
pub unsafe trait IAchtungBabyProfiler: IUnknown {}

impl IAchtungBabyProfiler_Impl for AchtungBabyProfiler_Impl {}

impl AchtungBabyProfiler {
    pub fn new() -> Self {
        Self {
            profiler_info: RefCell::new(None),
        }
    }

    pub unsafe fn set(&self,value:ICorProfilerInfo) {
        let mut info = self.profiler_info.borrow_mut();
        *info = Some(value);
    }
    pub fn is_none(&self) -> bool {
        self.profiler_info.borrow().is_none()
    }
}

impl ICorProfilerCallback_Impl for AchtungBabyProfiler_Impl {
    fn Initialize(&self, picorprofilerinfounk: windows_core::Ref<'_, windows_core::IUnknown>) -> windows_core::Result<()> {
        unsafe{MessageBoxW(
            None,
            w!("test"),
            w!("Initialize called"),
            Default::default(),
        );}
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
            let _ = self.profiler_info.borrow().as_ref().unwrap().SetEventMask(0x8);
                MessageBoxW(
                    None,
                    w!("test"),
                    w!("SetEventMask called"),
                    Default::default(),
                );
        };
        Ok(())
    }

    fn Shutdown(&self) -> windows_core::Result<()> {
        // ICorProfilerInfoの解放
        *self.profiler_info.borrow_mut() = None;
        Ok(())
    }

    fn AppDomainCreationStarted(&self, appdomainid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn AppDomainCreationFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        todo!()
    }

    fn AppDomainShutdownStarted(&self, appdomainid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn AppDomainShutdownFinished(&self, appdomainid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        todo!()
    }

    fn AssemblyLoadStarted(&self, assemblyid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn AssemblyLoadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        todo!()
    }

    fn AssemblyUnloadStarted(&self, assemblyid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn AssemblyUnloadFinished(&self, assemblyid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        todo!()
    }

    fn ModuleLoadStarted(&self, moduleid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ModuleLoadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        todo!()
    }

    fn ModuleUnloadStarted(&self, moduleid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ModuleUnloadFinished(&self, moduleid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        todo!()
    }

    fn ModuleAttachedToAssembly(&self, moduleid: usize, assemblyid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ClassLoadStarted(&self, classid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ClassLoadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        todo!()
    }

    fn ClassUnloadStarted(&self, classid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ClassUnloadFinished(&self, classid: usize, hrstatus: windows_core::HRESULT) -> windows_core::Result<()> {
        todo!()
    }

    fn FunctionUnloadStarted(&self, functionid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn JITCompilationStarted(&self, functionid: usize, fissafetoblock: windows_core::BOOL) -> windows_core::Result<()> {
        Ok(())
    }

    fn JITCompilationFinished(&self, functionid: usize, hrstatus: windows_core::HRESULT, fissafetoblock: windows_core::BOOL) -> windows_core::Result<()> {
        todo!()
    }

    fn JITCachedFunctionSearchStarted(&self, functionid: usize) -> windows_core::Result<windows_core::BOOL> {
        todo!()
    }

    fn JITCachedFunctionSearchFinished(&self, functionid: usize, result: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_JIT_CACHE) -> windows_core::Result<()> {
        todo!()
    }

    fn JITFunctionPitched(&self, functionid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn JITInlining(&self, callerid: usize, calleeid: usize) -> windows_core::Result<windows_core::BOOL> {
        todo!()
    }

    fn ThreadCreated(&self, threadid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ThreadDestroyed(&self, threadid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ThreadAssignedToOSThread(&self, managedthreadid: usize, osthreadid: u32) -> windows_core::Result<()> {
        todo!()
    }

    fn RemotingClientInvocationStarted(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn RemotingClientSendingMessage(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        todo!()
    }

    fn RemotingClientReceivingReply(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        todo!()
    }

    fn RemotingClientInvocationFinished(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn RemotingServerReceivingMessage(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        todo!()
    }

    fn RemotingServerInvocationStarted(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn RemotingServerInvocationReturned(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn RemotingServerSendingReply(&self, pcookie: *const windows_core::GUID, fisasync: windows_core::BOOL) -> windows_core::Result<()> {
        todo!()
    }

    fn UnmanagedToManagedTransition(&self, functionid: usize, reason: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_TRANSITION_REASON) -> windows_core::Result<()> {
        todo!()
    }

    fn ManagedToUnmanagedTransition(&self, functionid: usize, reason: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_TRANSITION_REASON) -> windows_core::Result<()> {
        todo!()
    }

    fn RuntimeSuspendStarted(&self, suspendreason: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_SUSPEND_REASON) -> windows_core::Result<()> {
        todo!()
    }

    fn RuntimeSuspendFinished(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn RuntimeSuspendAborted(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn RuntimeResumeStarted(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn RuntimeResumeFinished(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn RuntimeThreadSuspended(&self, threadid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn RuntimeThreadResumed(&self, threadid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn MovedReferences(&self, cmovedobjectidranges: u32, oldobjectidrangestart: *const usize, newobjectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::Result<()> {
        todo!()
    }

    fn ObjectAllocated(&self, objectid: usize, classid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ObjectsAllocatedByClass(&self, cclasscount: u32, classids: *const usize, cobjects: *const u32) -> windows_core::Result<()> {
        todo!()
    }

    fn ObjectReferences(&self, objectid: usize, classid: usize, cobjectrefs: u32, objectrefids: *const usize) -> windows_core::Result<()> {
        todo!()
    }

    fn RootReferences(&self, crootrefs: u32, rootrefids: *const usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionThrown(&self, thrownobjectid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionSearchFunctionEnter(&self, functionid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionSearchFunctionLeave(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionSearchFilterEnter(&self, functionid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionSearchFilterLeave(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionSearchCatcherFound(&self, functionid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionOSHandlerEnter(&self, __unused: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionOSHandlerLeave(&self, __unused: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionUnwindFunctionEnter(&self, functionid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionUnwindFunctionLeave(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionUnwindFinallyEnter(&self, functionid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionUnwindFinallyLeave(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionCatcherEnter(&self, functionid: usize, objectid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionCatcherLeave(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn COMClassicVTableCreated(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void, cslots: u32) -> windows_core::Result<()> {
        todo!()
    }

    fn COMClassicVTableDestroyed(&self, wrappedclassid: usize, implementediid: *const windows_core::GUID, pvtable: *const core::ffi::c_void) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionCLRCatcherFound(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn ExceptionCLRCatcherExecute(&self) -> windows_core::Result<()> {
        todo!()
    }
}


impl ICorProfilerCallback2_Impl for AchtungBabyProfiler_Impl {
    fn ThreadNameChanged(&self, threadid: usize, cchname: u32, name: &windows_core::PCWSTR) -> windows_core::Result<()> {
        todo!()
    }

    fn GarbageCollectionStarted(&self, cgenerations: i32, generationcollected: *const windows_core::BOOL, reason: windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_GC_REASON) -> windows_core::Result<()> {
        todo!()
    }

    fn SurvivingReferences(&self, csurvivingobjectidranges: u32, objectidrangestart: *const usize, cobjectidrangelength: *const u32) -> windows_core::Result<()> {
        todo!()
    }

    fn GarbageCollectionFinished(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn FinalizeableObjectQueued(&self, finalizerflags: u32, objectid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn RootReferences2(&self, crootrefs: u32, rootrefids: *const usize, rootkinds: *const windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_GC_ROOT_KIND, rootflags: *const windows::Win32::System::Diagnostics::ClrProfiling::COR_PRF_GC_ROOT_FLAGS, rootids: *const usize) -> windows_core::Result<()> {
        todo!()
    }

    fn HandleCreated(&self, handleid: usize, initialobjectid: usize) -> windows_core::Result<()> {
        todo!()
    }

    fn HandleDestroyed(&self, handleid: usize) -> windows_core::Result<()> {
        todo!()
    }
}