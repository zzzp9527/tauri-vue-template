
# Tauri Vue Template
Tauri + Vue3 + TailwindCSS

附带少量必要的插件，开箱即用



## Run Locally

Clone the project


```bash
  pnpm i
```

Start the server

```bash
  pnpm tauri dev
```

Build

```bash
  pnpm tauri build
```

## Console

Mac 上通过 Command + Option + I 打开 Console
Win 上通过 Ctrl + Shift + I 打开 Console

## Log

添加了 Log 插件，在不同系统有不同的日志存放位置

- Mac: {homeDir}/Library/Logs/{identifier}
- Win: {LocalAppData}/{identifier}/logs
- Linux: $HOME/.local/share/{identifier}/logs
