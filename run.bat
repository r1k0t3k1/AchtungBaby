set COR_ENABLE_PROFILING=1
set COR_PROFILER={5c8e9579-53b9-5a69-6a75-2d232518df35}
set COMPlus_ProfAPI_ProfilerCompatibilitySetting=EnableV2Profiler

REG ADD "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}" /f
REG ADD "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}\InprocServer32" /f
REG ADD "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}\InprocServer32" /ve /t REG_SZ /d "C:\Users\lab\Desktop\AchtungBaby\target\release\AchtungBaby.dll" /f

powershell

set COR_ENABLE_PROFILING=
set COR_PROFILER=
set COMPlus_ProfAPI_ProfilerCompatibilitySetting=

REG DELETE "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}" /f