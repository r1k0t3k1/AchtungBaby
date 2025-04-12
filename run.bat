set COR_ENABLE_PROFILING=1
set COR_PROFILER={5c8e9579-53b9-5a69-6a75-2d232518df35}

REG ADD "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}" /f
REG ADD "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}\InprocServer32" /f
REG ADD "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}\InprocServer32" /ve /t REG_SZ /d "C:\Users\lab\Desktop\AchtungBaby\target\debug\achtung_baby.dll" /f

powershell

set COR_ENABLE_PROFILING=
set COR_PROFILER=

REG DELETE "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}" /f