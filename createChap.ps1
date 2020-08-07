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
      $Command
    )
    "{
  `"watch`": `"*.js`",
  `"ext`": `"js`",
  `"ignore`": [`".git`"],
  `"exec`": `"cls && $Command`"
}" | Out-File .\nodemon.json
  }

  function New-Js {
    param (
      $Chap
    )
    "console.log(`"-----$Chap-----`");" | Out-File .\main.js
  }

  function New-Rust {
    param (
      $Chap
    )
    "fn main() {
    println!(`"-------$Chap-------`");
}" | Out-File .\src\main.rs
    "target
Cargo.lock" | Out-File .\.gitignore
  }

  # Create the nodemon.json file
  Set-Location $js  
  New-Nodemon -Command 'node main.js'
  New-Js -Chap $chap

  Set-Location ..\$rs
  New-Nodemon -Command 'cargo fmt && cargo run'
  New-Rust -Chap $chap 

  Set-Location ..
  
  Clear-Host

  Write-Host "Created new chapter $chap!"  
}

