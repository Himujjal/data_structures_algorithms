$chap = $args[0]

if ($chap.ToString().Length -gt 0 ) {
  mkdir $chap
  Set-Location $chap

  $js = $chap + '_js'
  $rs = $chap + '_rs'

  mkdir $js
  cargo new $rs

  function New-Nodemon {
    param (
      $Command,
      $Exts,
      $Watch
    )
    "{
  `"watch`": $Watch,
  `"ext`": `"$Exts`",
  `"ignore`": [`".git`"],
  `"exec`": `"cls && $Command`"
}" | Out-File .\nodemon.json
  }

  function New-Js {
    param (
      $Chap
    )
    "{ `"type`": `"module`" }" | Out-File .\package.json
    "console.log(`"||====================$Chap==================||`");" | Out-File .\main.js
  }

  function New-Rust {
    param (
      $Chap
    )
    "fn main() {
    println!(`"||===================$Chap======================||`");
}" | Out-File .\src\main.rs
    "target
Cargo.lock" | Out-File .\.gitignore
  }

  function New-StartScript {
  '$Lang = $args[0]
$Folder = [System.IO.Path]::GetFileName($(Get-Location))
Write-Host $Folder
cd ".\$Folder`_$Lang"
nodemon
Register-EngineEvent PowerShell.Exiting -Action {
  cd ..
}
' | Out-File .\start.ps1
  }

  # Create the nodemon.json file
  Set-Location $js  
  New-Nodemon -Exts "js" -Watch "`"*.js`"" -Command 'node main.js'
  New-Js -Chap $chap

  Set-Location ..\$rs
  New-Nodemon -Command 'cargo fmt && cargo run'
  New-Nodemon -Exts "toml,rs" -Watch "[`"src/*.rs`", `"Cargo.toml`"]" -Command 'cargo fmt && cargo run'
  New-Rust -Chap $chap 

  Set-Location ..
  New-StartScript
  Clear-Host
  Write-Host "Created new chapter $chap!"  
}

