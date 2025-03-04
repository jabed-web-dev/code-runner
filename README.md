# code-runner

build:\
  $ `cargo build --release`

uses:\
  $ `run --help`\
  Usage: --pause? --clear? \<program> \<script> [arguments...]

test:\
  \$ `run --pause node test/main.js arg1 arg2`\
  \# When run build exe from source like C,C++,Rust use with arg '-'\
  $ `run main.exe -`

  \# If your have args remove this '-'\
  $ `run main.exe arg1 arg2`

> **Access globally move run.exe to the env folder path/bin**\
> $ `run node main.js`
> 
> **Run with vscode code runner:**
```json
"code-runner.executorMap": {
    "javascript": "run --clear node $fullFileName"
}
```