param([string]$Cache = '', [switch]$Offline)
$ErrorActionPreference = 'Stop'
$nativeRoot = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
if (-not $Cache) { $Cache = Join-Path $nativeRoot 'target/native-sdk' }
New-Item -ItemType Directory -Path $Cache -Force | Out-Null
$packages = Get-Content -LiteralPath (Join-Path $nativeRoot 'native/sdk-packages.json') -Raw | ConvertFrom-Json
Add-Type -AssemblyName System.IO.Compression.FileSystem
foreach ($package in $packages) {
    $archive = Join-Path $Cache ($package.id + '.' + $package.version + '.nupkg')
    if (-not (Test-Path -LiteralPath $archive)) {
        if ($Offline) { throw "SDK cache missing: $($package.id)" }
        Invoke-WebRequest -Uri $package.url -OutFile $archive
    }
    if ((Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant() -ne $package.sha256) { throw 'SDK package integrity check failed.' }
    $directory = Join-Path $Cache ($package.id + '.' + $package.version)
    if (-not (Test-Path -LiteralPath $directory)) { [System.IO.Compression.ZipFile]::ExtractToDirectory($archive,$directory) }
}
$foundation = Join-Path $Cache 'microsoft.windowsappsdk.foundation.2.3.12'
$interactive = Join-Path $Cache 'microsoft.windowsappsdk.interactiveexperiences.2.1.9'
$sdk = Join-Path $Cache 'microsoft.windowsappsdk.2.5.1'
$cpp = Join-Path $Cache 'microsoft.windows.cppwinrt.2.0.240405.15/bin/cppwinrt.exe'
$unionRoot = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits/10/UnionMetadata'
$metadata = Get-ChildItem -LiteralPath $unionRoot -Directory | Where-Object Name -Match '^10\.' | Sort-Object Name -Descending | ForEach-Object { Join-Path $_.FullName 'Windows.winmd' } | Where-Object { Test-Path -LiteralPath $_ } | Select-Object -First 1
if (-not $metadata) { throw 'Windows SDK metadata is required.' }
$generated = Join-Path $Cache 'generated'
& $cpp -in $metadata -in (Join-Path $interactive 'metadata/10.0.17763.0') -ref (Join-Path $foundation 'metadata') -out $generated -base
if ($LASTEXITCODE -ne 0) { throw 'WinRT projection failed.' }
$vswhere = Join-Path ${env:ProgramFiles(x86)} 'Microsoft Visual Studio/Installer/vswhere.exe'
$visualStudio = & $vswhere -latest -products '*' -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if (-not $visualStudio) { throw 'MSVC C++ tools are required.' }
$output = Join-Path $nativeRoot 'resources/acrylic'
New-Item -ItemType Directory -Path $output -Force | Out-Null
$cmdFile = Join-Path $Cache 'compile-acrylic.cmd'
$cmdText = @"
@echo off
call "$visualStudio\VC\Auxiliary\Build\vcvars64.bat"
if errorlevel 1 exit /b 1
cl /nologo /std:c++20 /O2 /EHsc /utf-8 /MT /LD /experimental:deterministic /pathmap:"$env:USERPROFILE=/build-user" /I"$generated" /I"$foundation\include" "$nativeRoot\native\acrylic.cpp" /Fe"$output\official-acrylic.dll" /Fo"$Cache\acrylic.obj" /link /Brepro /IMPLIB:"$Cache\official-acrylic.lib" /LIBPATH:"$foundation\lib\native\x64" Microsoft.WindowsAppRuntime.Bootstrap.lib CoreMessaging.lib windowsapp.lib dwmapi.lib user32.lib
"@
if ($cmdText.ToCharArray() | Where-Object { [int]$_ -gt 127 }) { throw 'Build paths must be ASCII for this command file.' }
[System.IO.File]::WriteAllText($cmdFile,$cmdText,[System.Text.Encoding]::ASCII)
& cmd /c $cmdFile
if ($LASTEXITCODE -ne 0) { throw 'Native Acrylic compilation failed.' }
Copy-Item -LiteralPath (Join-Path $foundation 'runtimes/win-x64/native/Microsoft.WindowsAppRuntime.Bootstrap.dll') -Destination $output
Copy-Item -LiteralPath (Join-Path $sdk 'license.txt') -Destination (Join-Path $output 'SDK-LICENSE.txt')
Copy-Item -LiteralPath (Join-Path $sdk 'NOTICE.txt') -Destination (Join-Path $output 'SDK-NOTICE.txt')
$records = @('official-acrylic.dll','Microsoft.WindowsAppRuntime.Bootstrap.dll','SDK-LICENSE.txt','SDK-NOTICE.txt') | ForEach-Object { @{ file=$_;sha256=(Get-FileHash -LiteralPath (Join-Path $output $_) -Algorithm SHA256).Hash.ToLowerInvariant() } }
$manifest = @{sdk='2.5.1';runtimeMinimum='2.5.1.0';deployment='framework-dependent';files=@($records)} | ConvertTo-Json -Depth 5
[System.IO.File]::WriteAllText((Join-Path $output 'MANIFEST.json'),$manifest,[System.Text.UTF8Encoding]::new($false))
Write-Output 'Native Acrylic resources built and hashed.'
