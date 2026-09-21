[CmdletBinding()]
param(
  [Parameter(Mandatory = $true)]
  [ValidateSet('Inventory', 'Init', 'Build', 'Inspect')]
  [string]$Action
)

$ErrorActionPreference = 'Stop'
$repositoryRoot = Split-Path -Parent $PSScriptRoot
$studioRoot = Join-Path $repositoryRoot '.tools\android-studio-quail4-patch1\android-studio'
$buildJdkRoot = Join-Path $repositoryRoot '.tools\microsoft-jdk-21.0.12.1\jdk-21.0.12.1+1'
$sdkRoot = Join-Path $repositoryRoot '.tools\android-sdk'
$ndkVersion = '30.0.16248370'
$ndkRoot = Join-Path $sdkRoot "ndk\$ndkVersion"
$artifactRoot = Join-Path $repositoryRoot '.artifacts\android-m0'

function Assert-File([string]$Path, [string]$Label) {
  if (-not (Test-Path -LiteralPath $Path -PathType Leaf)) {
    throw "$Label is missing: $Path"
  }
}

function Assert-Directory([string]$Path, [string]$Label) {
  if (-not (Test-Path -LiteralPath $Path -PathType Container)) {
    throw "$Label is missing: $Path"
  }
}

function Get-Sha256([string]$Path) {
  $stream = [System.IO.File]::OpenRead($Path)
  try {
    $sha = [System.Security.Cryptography.SHA256]::Create()
    try {
      return ([System.BitConverter]::ToString($sha.ComputeHash($stream))).Replace('-', '').ToLowerInvariant()
    } finally {
      $sha.Dispose()
    }
  } finally {
    $stream.Dispose()
  }
}

function Assert-AndroidWebBundleIsolation {
  $distRoot = Join-Path $repositoryRoot 'dist'
  Assert-Directory $distRoot 'Android M0 web bundle'
  $forbidden = @(
    'api.openai.com',
    'generativelanguage.googleapis.com',
    'createLocalEvidenceStoreRuntime',
    'getAiRuntimeStatus',
    'initialize_sqlite',
    'desktop-schema-v5'
  )
  $bundleFiles = @(Get-ChildItem -LiteralPath $distRoot -File -Recurse)
  foreach ($token in $forbidden) {
    $match = $bundleFiles | Select-String -SimpleMatch -Pattern $token -List
    if ($match) {
      throw "Android M0 web bundle contains forbidden desktop/provider token '$token' in $($match.Path)."
    }
  }
  Write-Host 'Android M0 web bundle contains no forbidden desktop/provider tokens.'
}

Push-Location $repositoryRoot
try {
  . (Join-Path $PSScriptRoot 'use-local-dev-env.ps1')

  Assert-File (Join-Path $studioRoot 'jbr\bin\java.exe') 'Android Studio bundled JBR'
  Assert-File (Join-Path $buildJdkRoot 'bin\java.exe') 'Project-local Microsoft OpenJDK 21 build runtime'
  Assert-File (Join-Path $sdkRoot 'platform-tools\adb.exe') 'Android Platform-Tools'
  Assert-File (Join-Path $sdkRoot 'build-tools\36.0.0\apksigner.bat') 'Android APK signer'
  Assert-File (Join-Path $sdkRoot 'cmdline-tools\latest\bin\apkanalyzer.bat') 'Android APK analyzer'
  Assert-Directory $ndkRoot 'Android NDK'

  # Android Studio uses its bundled JBR 25. Gradle 8.14 does not support Java
  # 25, so packaging uses a separately verified, project-local JDK 21.
  $env:JAVA_HOME = $buildJdkRoot
  $env:ANDROID_HOME = $sdkRoot
  $env:ANDROID_SDK_ROOT = $sdkRoot
  $env:NDK_HOME = $ndkRoot
  $env:GRADLE_USER_HOME = Join-Path $repositoryRoot '.tools\gradle'
  $env:ANDROID_AVD_HOME = Join-Path $repositoryRoot '.artifacts\android-avd'
  $env:PATH = @(
    (Join-Path $sdkRoot 'platform-tools'),
    (Join-Path $sdkRoot 'cmdline-tools\latest\bin'),
    (Join-Path $sdkRoot 'emulator'),
    $env:PATH
  ) -join ';'

  if ($Action -eq 'Inventory') {
    Write-Host 'Android Studio bundled runtime:'
    & (Join-Path $studioRoot 'jbr\bin\java.exe') -version
    Write-Host 'Gradle build runtime:'
    & (Join-Path $env:JAVA_HOME 'bin\java.exe') -version
    & (Join-Path $sdkRoot 'platform-tools\adb.exe') version
    & (Join-Path $sdkRoot 'emulator\emulator.exe') -version 2>&1 | Select-Object -First 3
    & (Join-Path $sdkRoot 'cmdline-tools\latest\bin\sdkmanager.bat') --list_installed
    rustup target list --installed
    exit 0
  }

  if ($Action -eq 'Init') {
    if (Test-Path -LiteralPath (Join-Path $repositoryRoot 'src-tauri\gen\android')) {
      throw 'Android project already exists. Refusing to overwrite the reviewed generated surface.'
    }
    pnpm tauri android init --ci --skip-targets-install
    if ($LASTEXITCODE -ne 0) { throw "Tauri Android init failed with exit code $LASTEXITCODE." }
    exit 0
  }

  if ($Action -eq 'Build') {
    Assert-Directory (Join-Path $repositoryRoot 'src-tauri\gen\android') 'Generated Android project'
    node --test scripts/android-m0-contract.node-test.mjs
    if ($LASTEXITCODE -ne 0) { throw 'Android M0 contract tests failed before build.' }
    # The official Tauri command performs frontend/code generation and the
    # cargo-mobile NDK build. On Windows without Developer Mode it can fail
    # only at its final jniLibs symlink. That OS setting is outside M0's
    # automated authority, so this script permits exactly that failure and
    # finishes packaging from a normal copied library through Gradle.
    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
      $tauriOutput = @(& pnpm tauri android build --debug --target x86_64 2>&1)
      $tauriExit = $LASTEXITCODE
    } finally {
      $ErrorActionPreference = $previousErrorActionPreference
    }
    $tauriOutput | ForEach-Object { Write-Host $_ }
    Assert-AndroidWebBundleIsolation
    if ($tauriExit -eq 0) { exit 0 }

    $tauriText = $tauriOutput -join "`n"
    if ($tauriText -notmatch 'Creation symbolic link is not allowed') {
      throw "Tauri Android build failed before the bounded Windows symlink fallback (exit $tauriExit)."
    }

    $rustLibrary = Join-Path $repositoryRoot 'src-tauri\target\x86_64-linux-android\debug\liblife_os_lib.so'
    Assert-File $rustLibrary 'Tauri x86_64 Android native library'
    $jniDirectory = Join-Path $repositoryRoot 'src-tauri\gen\android\app\src\main\jniLibs\x86_64'
    New-Item -ItemType Directory -Force -Path $jniDirectory | Out-Null
    Copy-Item -LiteralPath $rustLibrary -Destination (Join-Path $jniDirectory 'liblife_os_lib.so') -Force

    Write-Host 'Tauri code generation and Rust build passed; using the no-elevation jniLibs copy fallback.' -ForegroundColor Yellow
    Push-Location (Join-Path $repositoryRoot 'src-tauri\gen\android')
    try {
      & (Join-Path $repositoryRoot 'src-tauri\gen\android\gradlew.bat') --no-daemon assembleX86_64Debug -x rustBuildX86_64Debug
      if ($LASTEXITCODE -ne 0) { throw "Gradle APK packaging failed with exit code $LASTEXITCODE." }
    } finally {
      Pop-Location
    }
    exit 0
  }

  New-Item -ItemType Directory -Force -Path $artifactRoot | Out-Null
  $apk = Get-ChildItem -LiteralPath (Join-Path $repositoryRoot 'src-tauri\gen\android\app\build\outputs\apk') -Filter '*.apk' -File -Recurse |
    Where-Object { $_.Name -match 'x86_64.*debug|debug.*x86_64' } |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First 1
  if (-not $apk) { throw 'No x86_64 debug APK was found. Run pnpm android:m0:build first.' }

  $apkAnalyzer = Join-Path $sdkRoot 'cmdline-tools\latest\bin\apkanalyzer.bat'
  $apkSigner = Join-Path $sdkRoot 'build-tools\36.0.0\apksigner.bat'
  $manifest = (& $apkAnalyzer manifest print $apk.FullName 2>&1) -join "`n"
  if ($LASTEXITCODE -ne 0) { throw 'Unable to print the packaged manifest.' }
  $summary = (& $apkAnalyzer apk summary $apk.FullName 2>&1) -join "`n"
  if ($LASTEXITCODE -ne 0) { throw 'Unable to inspect the APK summary.' }
  $signing = (& $apkSigner verify --verbose --print-certs $apk.FullName 2>&1) -join "`n"
  if ($LASTEXITCODE -ne 0) { throw 'APK signature verification failed.' }
  $packagedFiles = (& $apkAnalyzer files list $apk.FullName 2>&1) -join "`n"
  if ($LASTEXITCODE -ne 0) { throw 'Unable to inspect packaged files.' }
  $legacyBackupRules = (& $apkAnalyzer resources xml --file res/xml/backup_rules.xml $apk.FullName 2>&1) -join "`n"
  if ($LASTEXITCODE -ne 0) { throw 'Unable to inspect packaged legacy backup rules.' }
  $modernBackupRules = (& $apkAnalyzer resources xml --file res/xml/data_extraction_rules.xml $apk.FullName 2>&1) -join "`n"
  if ($LASTEXITCODE -ne 0) { throw 'Unable to inspect packaged Android 12+ backup rules.' }

  if ($manifest -notmatch 'package="com\.lifeos\.feasibility\.m0"') { throw 'Packaged application ID is not the temporary M0 identifier.' }
  $permissionNames = @([regex]::Matches($manifest, '<uses-permission[\s\S]*?android:name="([^"]+)"[\s\S]*?/>') | ForEach-Object { $_.Groups[1].Value })
  $allowedPermission = 'com.lifeos.feasibility.m0.DYNAMIC_RECEIVER_NOT_EXPORTED_PERMISSION'
  $unexpectedPermissions = @($permissionNames | Where-Object { $_ -ne $allowedPermission })
  if ($unexpectedPermissions.Count -gt 0) {
    throw "Packaged M0 contains unexpected Android permissions: $($unexpectedPermissions -join ', ')"
  }
  if ($manifest -notmatch 'android:allowBackup="false"') { throw 'Packaged M0 does not disable Android backup.' }
  if ($manifest -notmatch 'android:usesCleartextTraffic="false"') { throw 'Packaged M0 does not disable cleartext traffic.' }
  if ($manifest -notmatch 'android:dataExtractionRules="@ref/[^\"]+"') { throw 'Packaged M0 is missing Android 12+ data extraction rules.' }
  if ($manifest -notmatch 'android:fullBackupContent="@ref/[^\"]+"') { throw 'Packaged M0 is missing legacy backup rules.' }
  $nativeLibraries = @($packagedFiles -split "`r?`n" | Where-Object { $_ -match '^/lib/[^/]+/[^/]+\.so$' })
  if ($nativeLibraries.Count -ne 1 -or $nativeLibraries[0] -ne '/lib/x86_64/liblife_os_lib.so') {
    throw "Packaged M0 has an unexpected native ABI surface: $($nativeLibraries -join ', ')"
  }
  foreach ($domain in @('root', 'file', 'database', 'sharedpref', 'external', 'device_root', 'device_file', 'device_database', 'device_sharedpref')) {
    $domainPattern = 'domain="{0}"' -f [regex]::Escape($domain)
    if ($legacyBackupRules -notmatch $domainPattern) { throw "Legacy packaged backup rules do not exclude $domain." }
    if (([regex]::Matches($modernBackupRules, $domainPattern)).Count -ne 2) { throw "Modern packaged backup rules do not exclude $domain from both transfer paths." }
  }

  $hash = Get-Sha256 $apk.FullName
  $relativeApk = (Resolve-Path -LiteralPath $apk.FullName -Relative)
  $report = @(
    'Life OS Android Build Feasibility M0 - APK Inspection',
    "GeneratedAtUtc: $([DateTime]::UtcNow.ToString('o'))",
    "Apk: $relativeApk",
    "Bytes: $($apk.Length)",
    "SHA256: $hash",
    '',
    '=== APK Summary ===',
    $summary,
    '',
    '=== Signature ===',
    $signing,
    '',
    '=== Native Libraries ===',
    ($nativeLibraries -join "`n"),
    '',
    '=== Packaged Legacy Backup Rules ===',
    $legacyBackupRules,
    '',
    '=== Packaged Android 12+ Backup And Transfer Rules ===',
    $modernBackupRules,
    '',
    '=== Packaged Manifest ===',
    $manifest
  ) -join "`n"
  $reportPath = Join-Path $artifactRoot 'apk-inspection.txt'
  [System.IO.File]::WriteAllText($reportPath, $report, [System.Text.UTF8Encoding]::new($false))
  Write-Host $report
  Write-Host "Inspection evidence: $reportPath"
} finally {
  Pop-Location
}
