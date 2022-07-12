# 网络扫描图形化工具

---

## 功能描述
项目主要是辅助IT运维工作，而开发的。
IP扫描支持（扫描不同网段存在的IP、主机名、端口、等信息并提供调试）
摄像头扫描支持（如大华、海康威视、TPLink、睿视等品牌的扫描，并一键修改配置）

## 技术描述
UI用的是rust下的tauri web打包库，其web技术用的是WebAssembly(wasm)。

### 安装rust
```
https://www.rust-lang.org/learn/get-started
# 拉项目
git clone https://github.com/EternalNight996/rustscan-1379.git
# Rust windows 配置
cd rustscan-1379
cd cp .cargo.config c:/users/administrator/.cargo/config
```
### 运行项目
```
yarn install
yarn build
yarn dev
```

## 配置cargo与wasm
```
# cargo install --locked trunk
# rustup target add wasm32-unknown-unknown
# cargo install wasm-bindgen-cli
# cargo install wasm-pack --no-default-features
```

## Tauri Windows平台配置
```
# vs c++安装
# 安装 webview2 https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section
# 安装tauri模板
# npm create tauri-app
# cd network-manager-1379
# cargo install tauri-bundler --force
# cargo install tauri-cli --version "^1.0.0"
# npm run tauri init
```

## windows打包
```
# 依赖webview2， 下载runtime板.cab
https://developer.microsoft.com/en-us/microsoft-edge/webview2/#download-section
# 解压到tauri
expand .\Microsoft.WebView2.FixedVersionRuntime.98.0.1108.50.x64.cab -F:* ./tauri
# 修改配置文件
{
  "tauri": {
    "bundle": {
      "windows": {
        "webviewFixedRuntimePath": "./Microsoft.WebView2.FixedVersionRuntime.98.0.1108.50.x64/"
      }
    }
  }
}
```

