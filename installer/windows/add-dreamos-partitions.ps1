<#
.SYNOPSIS
  既存のデータディスクを縮小して、DreamOS用のパーティションを末尾に追加する (Windows)

.DESCRIPTION
  * 既存のNTFSボリュームを「縮小」するだけです(Windows標準の Resize-Partition)。既存のファイルは消えません。
  * 空いた領域に、DreamOS用(型GUID専用・未フォーマット)と、OS間の共有領域(exFAT)を作ります。
  * 既定は「確認だけ」。実行は -Execute と -ConfirmText "SHRINK <ディスク番号> <ドライブ文字> <GB>" の両方が必要です。
  * システムディスク/ブートディスク/USBは拒否します。管理者権限が必要です。
  * 縮小の前に必ずバックアップを取り、chkdsk でエラーが無いことを確認してください。

.EXAMPLE
  .\add-dreamos-partitions.ps1 -DiskNumber 0 -DriveLetter H -SizeGB 500
  .\add-dreamos-partitions.ps1 -DiskNumber 0 -DriveLetter H -SizeGB 500 -Execute -ConfirmText "SHRINK 0 H 500"
#>
[CmdletBinding()]
param(
  [Parameter(Mandatory)][int]$DiskNumber,
  [Parameter(Mandatory)][string]$DriveLetter,
  [int]$SizeGB = 500,
  [int]$SharedGB = 50,
  [switch]$Execute,
  [string]$ConfirmText = ""
)
$ErrorActionPreference = 'Stop'
$DreamType = '{D5EA3051-0A60-4D5E-9D2A-5E41AA00D0EA}'  # dream-os-disk の TYPE_DREAMOS
$DriveLetter = $DriveLetter.TrimEnd(':').ToUpper()

if ($SharedGB -ge $SizeGB) { throw "-SharedGB は -SizeGB より小さくしてください" }
$disk = Get-Disk -Number $DiskNumber
if ($disk.IsSystem -or $disk.IsBoot -or $disk.BusType -eq 'USB') { throw "ディスク $DiskNumber はシステム/ブート/USBディスクです。拒否します" }
if ($disk.PartitionStyle -ne 'GPT') { throw "GPTのディスクのみ対象です" }
$part = Get-Partition -DriveLetter $DriveLetter
if ($part.DiskNumber -ne $DiskNumber) { throw "$DriveLetter`: はディスク $DiskNumber にありません" }
$vol = Get-Volume -DriveLetter $DriveLetter
if ($vol.FileSystem -ne 'NTFS') { throw "NTFS以外は縮小しません ($($vol.FileSystem))" }

$sup = Get-PartitionSupportedSize -DriveLetter $DriveLetter
$need = [int64]$SizeGB * 1GB
$newSize = $part.Size - $need
$usedGB = [math]::Round(($vol.Size - $vol.SizeRemaining) / 1GB, 1)
Write-Host ("ディスク {0}: {1} ({2}GB)" -f $DiskNumber, $disk.FriendlyName, [math]::Round($disk.Size / 1GB))
Write-Host ("縮小対象: {0}:  現在 {1}GB / 使用 {2}GB / 縮小できる最小 {3}GB" -f $DriveLetter, [math]::Round($part.Size / 1GB, 1), $usedGB, [math]::Round($sup.SizeMin / 1GB, 1))
Write-Host ("縮小後  : {0}:  {1}GB  (データは保持。空き領域 {2}GB を DreamOS 用に確保)" -f $DriveLetter, [math]::Round($newSize / 1GB, 1), $SizeGB)
Write-Host ("作成    : DreamOS {0}GB (未フォーマット、ドライブ文字なし) + 共有 {1}GB (exFAT, ラベル DREAMSHARED)" -f ($SizeGB - $SharedGB), $SharedGB)
if ($newSize -lt $sup.SizeMin) { throw "そこまで縮小できません。-SizeGB を小さくするか、不要なファイルを減らしてください (最大 $([math]::Round(($part.Size - $sup.SizeMin) / 1GB, 1))GB)" }

if (-not $Execute) {
  Write-Host "確認だけで終了しました。実行するには -Execute -ConfirmText `"SHRINK $DiskNumber $DriveLetter $SizeGB`" を付けてください。"
  Write-Host "実行前にバックアップと chkdsk ${DriveLetter}: を済ませてください。"
  return
}
if ($ConfirmText -ne "SHRINK $DiskNumber $DriveLetter $SizeGB") { throw "-ConfirmText `"SHRINK $DiskNumber $DriveLetter $SizeGB`" が一致しません" }
$admin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
if (-not $admin) { throw "管理者として実行してください" }

Write-Host "縮小しています(大きいボリュームでは時間がかかります)..."
Resize-Partition -DriveLetter $DriveLetter -Size $newSize
$p2 = Get-Partition -DriveLetter $DriveLetter
$offset = $p2.Offset + $p2.Size
$align = 1MB
$offset = [int64]([math]::Ceiling($offset / $align) * $align)

Write-Host "DreamOS用パーティションを作成しています..."
$dream = New-Partition -DiskNumber $DiskNumber -Offset $offset -Size (([int64]($SizeGB - $SharedGB)) * 1GB) -GptType $DreamType
$off2 = $dream.Offset + $dream.Size
Write-Host "共有パーティションを作成しています..."
$shared = New-Partition -DiskNumber $DiskNumber -Offset $off2 -Size (([int64]$SharedGB) * 1GB - 8MB) -AssignDriveLetter
Format-Volume -Partition $shared -FileSystem exFAT -NewFileSystemLabel "DREAMSHARED" -Confirm:$false | Out-Null

Write-Host "完了:"
Get-Partition -DiskNumber $DiskNumber | Select-Object PartitionNumber, DriveLetter, Type, @{n = 'GB'; e = { [math]::Round($_.Size / 1GB, 1) } } | Format-Table -AutoSize
