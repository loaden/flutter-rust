
# 1. Rust 学习笔记

记忆力逐渐丧失，只有笔记才能避免原地踏步。

## 1.1. 环境配置

### 1.1.1. 安装配置 Rust

* 地址：<https://www.rust-lang.org/zh-CN/tools/install>
* 升级：`$ rustup update stable`
* 查看：`$ rustup show`
* 格式化：`$ rustup component add rustfmt`

### 1.1.2. VSCode 插件

* 插件安装
  > rust-analyzer

### 1.1.3. cargo 启用中科大源

* 编辑 .cargo/config.toml

```toml
[source.crates-io]
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index/"
[net]
git-fetch-with-cli = true

```

### 1.1.4. 可能缺失的依赖

> pkg-config openssl webkit2gtk

## 1.2. 工程配置

* 应用程序
  > cargo new rust-app
* 动态库
  > cargo new --lib rust-lib

</br>
</br>

# 2. Tauri 学习笔记

## 2.1. 环境配置

### 2.1.1. 安装配置 Tauri

> `cargo install create-tauri-app --locked`
> `cargo install tauri-cli`

### 2.1.2. nodejs 转换淘宝源和官方源

```shell
npm config set registry https://registry.npm.taobao.org/
npm config set registry https://registry.npmjs.org/

yarn config set registry https://registry.npm.taobao.org/
yarn config set registry https://registry.npmjs.org/
```

## 2.2. 工程配置

* 创建工程
  > `cargo create-tauri-app`

* 运行工程
  > `cargo tauri dev`

* 发布工程
  > `cargo tauri build`

</br>
</br>

# 3. Flutter 学习笔记

## 3.1. 环境配置

### 3.1.1. 安装配置 Flutter

* 地址：<https://flutter.cn/docs/get-started/install>
* 解压：$HOME/Dev/Flutter 或者 D:\Dev\Flutter
* 将解压路径添加到环境变量PATH中
* 查询版本：`$ flutter --version`
* 升级版本：`$ flutter upgrade`
* 检查配置：`$ flutter doctor`
* 检查位置：`$ where flutter dart`
* 安卓开发：<https://developer.android.google.cn/studio>

### 3.1.2. VSCode 插件

* 插件安装
  > Flutter 、Dart

### 3.1.3. 国内网络镜像

* 国内镜像地址，Fuck GFW：<https://flutter.cn/community/china>

  ```shell
  export PUB_HOSTED_URL=https://pub.flutter-io.cn
  export FLUTTER_STORAGE_BASE_URL=https://storage.flutter-io.cn
  ```

* Windows平台开始菜单搜索“环境变量”，添加上述环境变量。
* git 禁止设置代理，否则 `flutter upgrade` 失败

  ```shell
  git config --global --add remote.origin.proxy ""
  git config --global --get remote.origin.proxy
  ```

## 3.2. Flutter 平台相关

### 3.2.1. macOS 平台

* 需要 .zprofile 导出环境变量，否则 brew 更新困难
  > export HOMEBREW_NO_INSTALL_FROM_API=1
* macOS 与原生 iOS 代码交互
  > sudo gem install cocoapods
* macOS 需要通过AppStore安装 Xcode
* macOS 需要安装 clang
  > xcode-select --install

### 3.2.2. Android 平台

* 创建虚拟机图形加速选择：`Hardware - GLES 2.0`
* 安卓虚拟机 “关于” 页面连续点击 “Build number” 开启 “开发者模式”

### 3.2.3. Windows 平台

* Android SDK Tools 安装 Google USB Driver

## 3.3. Rust FFI 连接

### 3.3.1. 安装配置

* 官网：<https://github.com/fzyzcjy/flutter_rust_bridge>
* 文档：<https://cjycode.com/flutter_rust_bridge/index.html>

* 工具和依赖

  > ```shell
  > dart pub global activate ffigen
  > cargo install flutter_rust_bridge_codegen

### 3.3.2. 用法

* 转换成 Dart 代码
  > flutter_rust_bridge_codegen --rust-input core/src/api.rs --dart-output ui/lib/api_generated.dart

* 集成到 Flutter 中

  ```dart
  const base = 'core';
  final path = Platform.isWindows ? '$base.dll' : 'lib$base.so';
  late final dylib = Platform.isIOS
      ? DynamicLibrary.process()
      : Platform.isMacOS
          ? DynamicLibrary.executable()
          : DynamicLibrary.open(path);
  api = ReporthCoreImpl(dylib);
  ```

  * 先引用so文件，这里还要考虑windows平台，所以也可能是dll。
  * 应用后直接使用就行了，接口是根据api.rs生成的，所有出入参都有Dart对应的类（名字一样，类型可能有细微差别）。

## 3.4. 工程配置

* 创建应用程序
  > flutter create flutter_app
* 启动 iOS 模拟器
  > open -a Simulator
* 运行应用程序
  > flutter run
* 编译发布
  > flutter build windows
  > flutter build apk
