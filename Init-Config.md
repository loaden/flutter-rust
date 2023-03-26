# 安装配置笔记

记忆力逐渐丧失，只有笔记才能避免原地踏步。

## 一、平台相关额外说明

* macOS 需要 .zprofile 导出环境变量，否则 brew 更新困难
  > export HOMEBREW_NO_INSTALL_FROM_API=1

* macOS 需要安装 clang
  > xcode-select --install

## 二、Flutter + Rust 安装配置

### 1. 安装配置 Rust

* 地址：<https://www.rust-lang.org/zh-CN/tools/install>
* 升级：`$ rustup update`
* 查看：`$ rustup show`

### 2. 安装配置 Flutter

* 地址：<https://flutter.cn/docs/get-started/install>
* 解压：$HOME/Dev/flutter 或者 D:\Dev\flutter
* 将解压路径添加到环境变量PATH中
* 查询版本：`$ flutter --version`
* 升级版本：`$ flutter upgrade`
* 检查配置：`$ flutter doctor`
* 检查位置：`$ where flutter dart`

## 三、flutter_rust_bridge 学习笔记

### 1. 安装配置

* 官网：<https://github.com/fzyzcjy/flutter_rust_bridge>
* 文档：<https://cjycode.com/flutter_rust_bridge/index.html>

* 工具和依赖

  > ```shell
  > dart pub global activate ffigen
  > cargo install flutter_rust_bridge_codegen

### 2. 用法

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
