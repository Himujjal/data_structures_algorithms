$Lang = $args[0]
$Folder = [System.IO.Path]::GetFileName($(Get-Location))
Write-Host $Folder
cd ".\$Folder`_$Lang"
nodemon
Register-EngineEvent PowerShell.Exiting -Action {
  cd ..
}
