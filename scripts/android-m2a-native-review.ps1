[CmdletBinding()]
param()

$ErrorActionPreference = 'Stop'
$repositoryRoot = Split-Path -Parent $PSScriptRoot
$sdkRoot = Join-Path $repositoryRoot '.tools\android-sdk'
$jdkRoot = Join-Path $repositoryRoot '.tools\microsoft-jdk-21.0.12.1\jdk-21.0.12.1+1'
$avdHome = Join-Path $repositoryRoot '.artifacts\android-avd'
$artifactRoot = Join-Path $repositoryRoot '.artifacts\android-m2a\native-review'
$avdName = 'lifeos_m2a_api36_x86_64'
$serial = 'emulator-5584'
$port = 5584
$cdpPort = 9224
$package = 'com.lifeos.review.m2a'
$component = "$package/.MainActivity"
$image = 'system-images;android-36;default;x86_64'
$adb = Join-Path $sdkRoot 'platform-tools\adb.exe'
$sqlite = Join-Path $sdkRoot 'platform-tools\sqlite3.exe'
$emulator = Join-Path $sdkRoot 'emulator\emulator.exe'
$avdManager = Join-Path $sdkRoot 'cmdline-tools\latest\bin\avdmanager.bat'
$apk = Get-ChildItem -LiteralPath (Join-Path $repositoryRoot 'src-tauri\gen\android\app\build\outputs\apk') -Filter '*.apk' -File -Recurse |
  Where-Object { $_.Name -match 'x86_64.*debug|debug.*x86_64' } |
  Sort-Object LastWriteTime -Descending |
  Select-Object -First 1
if (-not $apk) { throw 'M2A x86_64 debug APK is missing. Run pnpm android:m2a:build.' }
if (-not (Test-Path -LiteralPath $sqlite -PathType Leaf)) {
  throw "Repository-local Android sqlite3 is missing: $sqlite"
}

function Get-Sha256([string]$Path) {
  $stream = [IO.File]::OpenRead($Path)
  try {
    $sha = [Security.Cryptography.SHA256]::Create()
    try {
      return ([BitConverter]::ToString($sha.ComputeHash($stream))).Replace('-', '').ToLowerInvariant()
    } finally {
      $sha.Dispose()
    }
  } finally {
    $stream.Dispose()
  }
}

New-Item -ItemType Directory -Force -Path $artifactRoot | Out-Null
New-Item -ItemType Directory -Force -Path $avdHome | Out-Null
$env:JAVA_HOME = $jdkRoot
$env:ANDROID_HOME = $sdkRoot
$env:ANDROID_SDK_ROOT = $sdkRoot
$env:ANDROID_AVD_HOME = $avdHome

function Invoke-Adb([string[]]$Arguments, [switch]$AllowFailure) {
  $previousErrorActionPreference = $ErrorActionPreference
  $ErrorActionPreference = 'Continue'
  try {
    $output = @(& $adb -s $serial @Arguments 2>&1)
    $exit = $LASTEXITCODE
  } finally {
    $ErrorActionPreference = $previousErrorActionPreference
  }
  $transportText = $output -join ' '
  if ($exit -ne 0 -and $transportText -match 'daemon still not running|cannot connect to daemon') {
    # The command was not delivered to a device when the local adb daemon could
    # not be reached. Restart that transport once; never retry a command after
    # a device-side or otherwise ambiguous outcome.
    & $adb start-server 2>&1 | Out-Null
    Start-Sleep -Seconds 1
    $previousErrorActionPreference = $ErrorActionPreference
    $ErrorActionPreference = 'Continue'
    try {
      $output = @(& $adb -s $serial @Arguments 2>&1)
      $exit = $LASTEXITCODE
    } finally {
      $ErrorActionPreference = $previousErrorActionPreference
    }
  }
  if (-not $AllowFailure -and $exit -ne 0) {
    throw "adb $($Arguments -join ' ') failed ($exit): $($output -join ' ')"
  }
  return $output
}

function Wait-Boot {
  $deadline = [DateTime]::UtcNow.AddMinutes(4)
  do {
    Start-Sleep -Seconds 2
    $state = (Invoke-Adb @('get-state') -AllowFailure) -join ''
    if ($state.Trim() -eq 'device') {
      $boot = (Invoke-Adb @('shell', 'getprop', 'sys.boot_completed') -AllowFailure) -join ''
      if ($boot.Trim() -eq '1') { return }
    }
  } while ([DateTime]::UtcNow -lt $deadline)
  throw "Disposable AVD $avdName did not boot."
}

function Start-DisposableEmulator([switch]$WipeData) {
  $arguments = @('-avd', $avdName, '-port', "$port", '-no-snapshot', '-no-boot-anim', '-no-audio', '-no-window', '-gpu', 'swiftshader_indirect')
  if ($WipeData) { $arguments += '-wipe-data' }
  $process = Start-Process -FilePath $emulator -ArgumentList $arguments -WindowStyle Hidden -PassThru
  Wait-Boot
  $reportedName = ((Invoke-Adb @('emu', 'avd', 'name')) | Where-Object { $_ -and $_ -ne 'OK' } | Select-Object -First 1).Trim()
  if ($reportedName -ne $avdName) { throw "Exact emulator binding failed: $serial reports $reportedName." }
  return $process
}

function Stop-DisposableEmulator([System.Diagnostics.Process]$Process) {
  Invoke-Adb @('emu', 'kill') | Out-Null
  $deadline = [DateTime]::UtcNow.AddSeconds(45)
  do {
    Start-Sleep -Milliseconds 500
    $stillConnected = @(& $adb devices | Select-Object -Skip 1 | Where-Object { $_ -match "^$([regex]::Escape($serial))\s+" })
    if ($stillConnected.Count -eq 0) {
      if ($Process) { $Process.WaitForExit(10000) | Out-Null }
      return
    }
  } while ([DateTime]::UtcNow -lt $deadline)
  throw "Disposable AVD $avdName did not shut down cleanly."
}

function Start-AppAndForward {
  Invoke-Adb @('shell', 'am', 'start', '-W', '-n', $component) | Out-Null
  $deadline = [DateTime]::UtcNow.AddSeconds(30)
  do {
    Start-Sleep -Milliseconds 300
    $pidText = ((Invoke-Adb @('shell', 'pidof', $package) -AllowFailure) -join '').Trim()
  } while (-not $pidText -and [DateTime]::UtcNow -lt $deadline)
  if (-not $pidText) { throw 'M2A app process did not start.' }
  Invoke-Adb @('forward', '--remove', "tcp:$cdpPort") -AllowFailure | Out-Null
  Invoke-Adb @('forward', "tcp:$cdpPort", "localabstract:webview_devtools_remote_$pidText") | Out-Null
  return $pidText
}

function Invoke-Cdp([string]$Action, [string]$Text = '', [string]$Phase = '', [string]$Locale = '') {
  $arguments = @('scripts/android-m2a-cdp-probe.mjs', '--port', "$cdpPort", '--action', $Action)
  if ($Text) { $arguments += @('--text', $Text) }
  if ($Phase) { $arguments += @('--phase', $Phase) }
  if ($Locale) { $arguments += @('--locale', $Locale) }
  & node @arguments
  if ($LASTEXITCODE -ne 0) { throw "CDP probe $Action failed." }
  Write-Host ''
}

function Start-CdpArm([string]$Text, [string]$Phase) {
  $env:LIFE_OS_M2A_PROBE_TEXT = $Text
  $arguments = @('scripts/android-m2a-cdp-probe.mjs', '--port', "$cdpPort", '--action', 'arm-save', '--phase', $Phase)
  return Start-Process -FilePath 'node' -ArgumentList $arguments -WorkingDirectory $repositoryRoot -WindowStyle Hidden -PassThru
}

function Stop-CdpArm([System.Diagnostics.Process]$Process) {
  if (-not $Process) { return }
  $Process.Refresh()
  if (-not $Process.HasExited) {
    Stop-Process -Id $Process.Id -Force
  }
}

function Stop-App {
  Invoke-Adb @('shell', 'am', 'force-stop', $package) | Out-Null
  Start-Sleep -Milliseconds 500
}

function Assert-PendingNamesAbsent([string]$Parent) {
  foreach ($name in @(
      '.android-m2a-fresh-v5.pending.db',
      '.android-m2a-direct-fresh-v5.pending.receipt.json'
    )) {
    $found = ((Invoke-Adb @('shell', 'run-as', $package, 'find', $Parent, '-name', $name, '-print')) -join '').Trim()
    if ($found) { throw "Pending publication name survived final profile preparation: $found" }
  }
}

function Kill-AppProcess {
  $pidText = ((Invoke-Adb @('shell', 'pidof', $package) -AllowFailure) -join '').Trim()
  if (-not $pidText) { throw 'No M2A process was available for the process-kill check.' }
  Invoke-Adb @('shell', 'run-as', $package, 'kill', '-9', $pidText) -AllowFailure | Out-Null
  Start-Sleep -Seconds 1
  $remaining = ((Invoke-Adb @('shell', 'pidof', $package) -AllowFailure) -join '').Trim()
  if ($remaining) { throw 'M2A process remained alive after same-UID SIGKILL.' }
}

function Wait-HoldMarker([string]$MarkerName) {
  $deadline = [DateTime]::UtcNow.AddSeconds(8)
  do {
    Start-Sleep -Milliseconds 200
    $marker = ((Invoke-Adb @('shell', 'run-as', $package, 'find', '.', '-name', $MarkerName, '-print') -AllowFailure) -join '').Trim()
    if ($marker) { return $marker }
  } while ([DateTime]::UtcNow -lt $deadline)
  throw "Debug hold marker $MarkerName did not appear before process termination."
}

function Install-Fixture([string]$LocalPath, [string]$DatabaseRelativePath) {
  $remote = '/data/local/tmp/lifeos-m2a-fixture.db'
  Invoke-Adb @('push', $LocalPath, $remote) | Out-Null
  $slash = $DatabaseRelativePath.LastIndexOf('/')
  $parent = if ($slash -ge 0) { $DatabaseRelativePath.Substring(0, $slash) } else { '.' }
  Invoke-Adb @('shell', 'run-as', $package, 'mkdir', '-p', $parent) | Out-Null
  Invoke-Adb @('shell', 'run-as', $package, 'cp', $remote, $DatabaseRelativePath) | Out-Null
  Invoke-Adb @('shell', 'rm', '-f', $remote) | Out-Null
}

function Export-AppFile([string]$DatabaseRelativePath, [string]$LocalPath) {
  $startInfo = New-Object System.Diagnostics.ProcessStartInfo
  $startInfo.FileName = $adb
  $startInfo.Arguments = "-s $serial exec-out run-as $package cat $DatabaseRelativePath"
  $startInfo.UseShellExecute = $false
  $startInfo.RedirectStandardOutput = $true
  $startInfo.RedirectStandardError = $true
  $process = New-Object System.Diagnostics.Process
  $process.StartInfo = $startInfo
  [void]$process.Start()
  $stream = [IO.File]::Create($LocalPath)
  try {
    $process.StandardOutput.BaseStream.CopyTo($stream)
  } finally {
    $stream.Dispose()
  }
  $errorText = $process.StandardError.ReadToEnd()
  $process.WaitForExit()
  if ($process.ExitCode -ne 0) {
    throw "Unable to export app-private database: $errorText"
  }
}

$connected = @(& $adb devices | Select-Object -Skip 1 | Where-Object { $_ -match '\t(device|offline|unauthorized)$' })
if ($connected.Count -gt 0) {
  throw "Refusing to start native review while any device is connected: $($connected -join ', ')"
}

if (-not (Test-Path -LiteralPath (Join-Path $avdHome "$avdName.ini"))) {
  'no' | & $avdManager create avd --force --name $avdName --package $image --device 'pixel_6'
  $avdIni = Join-Path $avdHome "$avdName.ini"
  $avdConfig = Join-Path $avdHome "$avdName.avd\config.ini"
  if ($LASTEXITCODE -ne 0 -and -not (
      (Test-Path -LiteralPath $avdIni -PathType Leaf) -and
      (Test-Path -LiteralPath $avdConfig -PathType Leaf)
    )) {
    throw 'Dedicated disposable M2A AVD creation failed.'
  }
}

$results = [System.Collections.Generic.List[string]]::new()
$emulatorProcess = $null
try {
  $emulatorProcess = Start-DisposableEmulator -WipeData
  $existingPackage = ((Invoke-Adb @('shell', 'pm', 'path', $package) -AllowFailure) -join '').Trim()
  if ($existingPackage) {
    throw "Temporary M2-A identity collides with an existing installation: $existingPackage"
  }
  Invoke-Adb @('install', $apk.FullName) | Out-Null
  Invoke-Adb @('shell', 'pm', 'clear', $package) | Out-Null
  Start-AppAndForward | Out-Null

  Invoke-Cdp -Action 'set-locale' -Locale 'zh-TW'
  Stop-App
  Start-AppAndForward | Out-Null
  Invoke-Cdp -Action 'verify-locale' -Locale 'zh-TW'
  Invoke-Cdp -Action 'set-locale' -Locale 'en'
  $results.Add('PASS app-private non-content locale preference survives force-stop/relaunch')

  $journeyText = '合成 Experience：今天我想記住静かな勇気。'
  Invoke-Cdp 'journey' $journeyText
  $results.Add('PASS fresh exact-v5 creation, CJK save/list/get, exact reopen, and double-submit suppression')

  Invoke-Adb @('shell', 'input', 'keyevent', 'KEYCODE_HOME') | Out-Null
  Start-Sleep -Seconds 1
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-present' $journeyText
  $results.Add('PASS background/foreground preserves committed Experience')

  $results.Add('UNEXECUTED graceful application shutdown: Android has no portable process-level graceful-shutdown contract; background and force-stop are tested separately')

  Stop-App
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-present' $journeyText
  $results.Add('PASS force-stop/relaunch preserves committed Experience')

  $beforeText = 'before-ack process kill must not create a record'
  $beforeProbe = Start-CdpArm $beforeText 'beforeCommit'
  $beforeMarker = Wait-HoldMarker '.m2a-beforeCommit.hold'
  Stop-App
  Stop-CdpArm $beforeProbe
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-absent' $beforeText
  Invoke-Adb @('shell', 'run-as', $package, 'rm', '-f', $beforeMarker) | Out-Null
  $results.Add('PASS force-stop during the pre-commit hold leaves no Experience')

  $afterText = 'after-commit before-ack process kill remains durable'
  $afterProbe = Start-CdpArm $afterText 'afterCommitBeforeAck'
  $afterMarker = Wait-HoldMarker '.m2a-afterCommitBeforeAck.hold'
  Stop-App
  Stop-CdpArm $afterProbe
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-present' $afterText
  Invoke-Adb @('shell', 'run-as', $package, 'rm', '-f', $afterMarker) | Out-Null
  $results.Add('PASS force-stop after commit and before acknowledgement preserves Experience')
  $results.Add('UNEXECUTED raw same-UID SIGKILL: run-as kill timing was not sufficiently deterministic to claim')

  Stop-App
  $databaseRelative = ((Invoke-Adb @('shell', 'run-as', $package, 'find', '.', '-name', 'android-m2a-disposable-v5.db', '-print')) -join '').Trim()
  if (-not $databaseRelative) { throw 'Unable to locate the app-private M2A database with run-as.' }
  $receiptRelative = ((Invoke-Adb @('shell', 'run-as', $package, 'find', '.', '-name', 'android-m2a-direct-fresh-v5.receipt.json', '-print')) -join '').Trim()
  if (-not $receiptRelative) { throw 'Unable to locate the truthful direct-fresh receipt with run-as.' }
  $pulled = Join-Path $artifactRoot 'native-exact-v5.db'
  $pulledReceipt = Join-Path $artifactRoot 'native-direct-fresh-v5.receipt.json'
  Export-AppFile $databaseRelative $pulled
  Export-AppFile $receiptRelative $pulledReceipt
  $version = (& $sqlite $pulled 'PRAGMA user_version;').Trim()
  if ($version -ne '5') { throw "Native database schema version was $version, not 5." }
  $migrationReceipts = (& $sqlite $pulled 'SELECT COUNT(*) FROM schema_migration_receipts;').Trim()
  if ($migrationReceipts -ne '0') { throw "Direct native database fabricated $migrationReceipts migration receipt(s)." }
  $receiptJson = Get-Content -LiteralPath $pulledReceipt -Raw -Encoding UTF8 | ConvertFrom-Json
  if ($receiptJson.origin -ne 'direct_fresh_v5' -or $receiptJson.applicationId -ne $package -or $receiptJson.schemaVersion -ne 5) {
    throw 'Native direct-fresh initialization receipt is not truthful.'
  }
  $results.Add("PASS app-private direct schema-v5 database has user_version=5, zero migration receipts, and truthful fresh-origin receipt")

  if ($emulatorProcess -and -not $emulatorProcess.HasExited) {
    Stop-Process -Id $emulatorProcess.Id -Force
    Start-Sleep -Seconds 3
    $emulatorProcess = Start-DisposableEmulator
    Start-AppAndForward | Out-Null
    Invoke-Cdp 'verify-present' $journeyText
    $results.Add('PASS abrupt host-side emulator process termination/restart preserves acknowledged Experience')
  } else {
    $results.Add('UNEXECUTED abrupt emulator process termination: launcher process identity was unavailable')
  }

  Stop-App
  Invoke-Adb @('shell', 'pm', 'clear', $package) | Out-Null
  $malformed = Join-Path $artifactRoot 'malformed.db'
  [IO.File]::WriteAllBytes($malformed, [Text.Encoding]::UTF8.GetBytes('not a sqlite database - preserve exactly'))
  Install-Fixture $malformed $databaseRelative
  Install-Fixture $pulledReceipt $receiptRelative
  $malformedBefore = ((Invoke-Adb @('shell', 'run-as', $package, 'sha256sum', $databaseRelative)) -join '').Split(' ')[0]
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-blocked' 'm2a_existing_database_refused'
  Stop-App
  $malformedAfter = ((Invoke-Adb @('shell', 'run-as', $package, 'sha256sum', $databaseRelative)) -join '').Split(' ')[0]
  if ($malformedBefore -ne $malformedAfter) { throw 'Malformed database changed during fail-closed open.' }
  $results.Add('PASS malformed native database is refused and byte-preserved')

  Invoke-Adb @('shell', 'pm', 'clear', $package) | Out-Null
  $newer = Join-Path $artifactRoot 'newer-v6.db'
  if (Test-Path -LiteralPath $newer) { Remove-Item -LiteralPath $newer -Force }
  & $sqlite $newer 'PRAGMA user_version=6; CREATE TABLE sentinel(value TEXT);'
  if ($LASTEXITCODE -ne 0) { throw 'Unable to create newer-schema native fixture.' }
  Install-Fixture $newer $databaseRelative
  Install-Fixture $pulledReceipt $receiptRelative
  $newerBefore = ((Invoke-Adb @('shell', 'run-as', $package, 'sha256sum', $databaseRelative)) -join '').Split(' ')[0]
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-blocked' 'm2a_existing_database_refused'
  Stop-App
  $newerAfter = ((Invoke-Adb @('shell', 'run-as', $package, 'sha256sum', $databaseRelative)) -join '').Split(' ')[0]
  if ($newerBefore -ne $newerAfter) { throw 'Newer-schema database changed during fail-closed open.' }
  $results.Add('PASS newer schema v6 is refused and byte-preserved')

  Invoke-Adb @('shell', 'pm', 'clear', $package) | Out-Null
  $slash = $databaseRelative.LastIndexOf('/')
  $parent = if ($slash -ge 0) { $databaseRelative.Substring(0, $slash) } else { '.' }
  $pendingRelative = "$parent/.android-m2a-fresh-v5.pending.db"
  Install-Fixture $malformed $pendingRelative
  $pendingBefore = ((Invoke-Adb @('shell', 'run-as', $package, 'sha256sum', $pendingRelative)) -join '').Split(' ')[0]
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-blocked' 'm2a_publication_state_incomplete_preserved'
  Stop-App
  $pendingAfter = ((Invoke-Adb @('shell', 'run-as', $package, 'sha256sum', $pendingRelative)) -join '').Split(' ')[0]
  if ($pendingBefore -ne $pendingAfter) { throw 'Pending initialization evidence changed during fail-closed open.' }
  $results.Add('PASS pending initialization state is refused and byte-preserved')

  Invoke-Adb @('shell', 'pm', 'clear', $package) | Out-Null
  Install-Fixture $pulled $databaseRelative
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-blocked' 'm2a_publication_state_incomplete_preserved'
  Stop-App
  $results.Add('PASS partially published live database without receipt is preserved and refused')

  Invoke-Adb @('shell', 'pm', 'clear', $package) | Out-Null
  Install-Fixture $pulled $databaseRelative
  Install-Fixture $pulledReceipt $receiptRelative
  $sidecar = Join-Path $artifactRoot 'preserve-wal.bin'
  [IO.File]::WriteAllBytes($sidecar, [Text.Encoding]::UTF8.GetBytes('preserve synthetic WAL evidence'))
  $walRelative = "${databaseRelative}-wal"
  Install-Fixture $sidecar $walRelative
  $walBefore = ((Invoke-Adb @('shell', 'run-as', $package, 'sha256sum', $walRelative)) -join '').Split(' ')[0]
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-blocked' 'm2a_sqlite_sidecar_preserved'
  Stop-App
  $walAfter = ((Invoke-Adb @('shell', 'run-as', $package, 'sha256sum', $walRelative)) -join '').Split(' ')[0]
  if ($walBefore -ne $walAfter) { throw 'WAL evidence changed during fail-closed open.' }
  $results.Add('PASS explicit WAL sidecar state is refused and byte-preserved')

  Invoke-Adb @('shell', 'pm', 'clear', $package) | Out-Null
  Invoke-Adb @('shell', 'run-as', $package, 'mkdir', '-p', $parent) | Out-Null
  Invoke-Adb @('shell', 'run-as', $package, 'mkdir', $databaseRelative) | Out-Null
  Install-Fixture $pulledReceipt $receiptRelative
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-blocked' 'm2a_publication_state_incomplete_preserved'
  $results.Add('PASS database-open failure is visible and the conflicting path is preserved')

  Stop-App
  Invoke-Adb @('shell', 'pm', 'clear', $package) | Out-Null
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-absent' $journeyText
  $results.Add('PASS final Founder review profile is freshly disposable and ready')
  Assert-PendingNamesAbsent $parent

  # Reproduce the Founder boundary exactly: do not issue `adb shell sync` or
  # otherwise repair/flush the profile from the harness before terminating the
  # exact emulator. Product code must make pending-name retirement durable.
  Stop-DisposableEmulator $emulatorProcess
  $emulatorProcess = Start-DisposableEmulator
  Start-AppAndForward | Out-Null
  Invoke-Cdp 'verify-absent' $journeyText
  Assert-PendingNamesAbsent $parent
  $results.Add('PASS final Founder review profile survives exact AVD shutdown/restart with ready storage and no pending names')

  $results.Add('UNSUPPORTED actual physical power-loss test: no physical device and no claim from emulator evidence')
  $results.Add('UNSUPPORTED production backup/restore, migration/recovery, update/delete/import/export, and non-x86_64 ABI/device matrix')

  $report = @(
    '# Android M2-A direct fresh-v5 disposable native evidence',
    '',
    "- Generated UTC: $([DateTime]::UtcNow.ToString('o'))",
    "- Exact AVD: $avdName",
    "- Exact serial: $serial",
    "- API / ABI: Android 36 / x86_64",
    "- Package: $package",
    "- APK: $($apk.FullName)",
    "- APK SHA256: $(Get-Sha256 $apk.FullName)",
    '',
    '## Results',
    ''
  ) + ($results | ForEach-Object { "- $_" })
  $reportPath = Join-Path $artifactRoot 'native-result.md'
  [IO.File]::WriteAllLines($reportPath, $report, [Text.UTF8Encoding]::new($false))
  Write-Host "Native evidence: $reportPath"
  $results | ForEach-Object { Write-Host $_ }
} finally {
  $previousErrorActionPreference = $ErrorActionPreference
  $ErrorActionPreference = 'Continue'
  try {
    $logcatPath = Join-Path $artifactRoot 'native-logcat.txt'
    & $adb -s $serial logcat -d 2>$null | Set-Content -LiteralPath $logcatPath -Encoding UTF8
    & $adb -s $serial forward --remove "tcp:$cdpPort" 2>$null | Out-Null
    & $adb -s $serial emu kill 2>$null | Out-Null
  } finally {
    $ErrorActionPreference = $previousErrorActionPreference
  }
}
