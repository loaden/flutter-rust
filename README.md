
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

### 1.1.4. Watt Toolkit 访问 github

* 下载地址：<https://github.com/BeyondDimension/SteamTools>
* Windows查看代理端口号：Windows设置-网络-代理，可以查看到端口号26501
* 开启pac代理模式，并一键加速
* 支持https克隆源码库

```shell
git config --global http.proxy 127.0.0.1:26501
git config --global http.sslverify false
```

* 如需重置，可以

```shell
git config --global --unset http.proxy
git config --global --unset http.sslverify
```

* ssh改走https协议，走443端口，修改 .ssh/config

```shell
Host github.com
    Hostname ssh.github.com
    Port 443
    User git
```

## 1.2. Rust 平台相关

### 1.2.1. Unix 平台

* 可能缺失的依赖
  > pkg-config libssl-dev openssl

### 1.2.2. Linux 平台

* 可能缺失的依赖
  > gcc glibc llvm-libs

### 1.2.3. Android 平台

* 编译目标
  > rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

* 安装cargo-ndk
  > cargo install cargo-ndk
* 可能需要配置NDK环境变量：`ANDROID_HOME` or `ANDROID_NDK_HOME`

### 1.2.4. iOS 平台

* 编译目标

  ```shell
  # 64 bit targets (real device & simulator):
  rustup target add aarch64-apple-ios x86_64-apple-ios

  # New simulator target for Xcode 12 and later
  rustup target add aarch64-apple-ios-sim
  ```

#### 1.2.4.1. OpenSSL for iOS

* 一键编译包：`https://github.com/x2on/OpenSSL-for-iPhone`
* 执行：`./build-libssl.sh --targets="ios-sim-cross-x86_64 ios-sim-cross-arm64 ios-cross-arm64"`
* 添加环境变量，注意target，模拟器取决于host的CPU指令集

#### 1.2.4.2. 交叉编译

* 查看工具链和编译目标

```shell
rustup toolchain list
rustup target list
```

* 指定linker：交叉编译通常需要指定对应的编译器，否则 Rust 默认调用平台安装的 cc 编译器
  > cargo build --target x86_64-apple-ios

#### 1.2.4.3. iOS 交叉编译openssl

```shell
# 编译目标iOS模拟器，host是英特尔CPU的macOS：x64
export OPENSSL_LIB_DIR=$HOME/.dev/projects/OpenSSL-for-iPhone/bin/iPhoneSimulator16.2-x86_64.sdk/lib
export OPENSSL_INCLUDE_DIR=$HOME/.dev/projects/OpenSSL-for-iPhone/bin/iPhoneSimulator16.2-x86_64.sdk/include
export PKG_CONFIG_PATH=$HOME/.dev/projects/OpenSSL-for-iPhone/bin/iPhoneSimulator16.2-x86_64.sdk/lib
echo $PKG_CONFIG_PATH
echo $OPENSSL_LIB_DIR
echo $OPENSSL_INCLUDE_DIR
read
cargo build --manifest-path=native/Cargo.toml --target=x86_64-apple-ios

# 编译目标iOS模拟器，host是M1的macOS：arm64
export OPENSSL_LIB_DIR=$HOME/.dev/projects/OpenSSL-for-iPhone/bin/iPhoneSimulator16.2-arm64.sdk/lib
export OPENSSL_INCLUDE_DIR=$HOME/.dev/projects/OpenSSL-for-iPhone/bin/iPhoneSimulator16.2-arm64.sdk/include
export PKG_CONFIG_PATH=$HOME/.dev/projects/OpenSSL-for-iPhone/bin/iPhoneSimulator16.2-arm64.sdk/lib
echo $PKG_CONFIG_PATH
echo $OPENSSL_LIB_DIR
echo $OPENSSL_INCLUDE_DIR
read
cargo build --manifest-path=native/Cargo.toml --target=aarch64-apple-ios-sim

# 编译目标真机
export OPENSSL_LIB_DIR=$HOME/.dev/projects/OpenSSL-for-iPhone/bin/iPhoneOS16.2-arm64.sdk/lib
export OPENSSL_INCLUDE_DIR=$HOME/.dev/projects/OpenSSL-for-iPhone/bin/iPhoneOS16.2-arm64.sdk/include
export PKG_CONFIG_PATH=$HOME/.dev/projects/OpenSSL-for-iPhone/bin/iPhoneOS16.2-arm64.sdk/lib
echo $PKG_CONFIG_PATH
echo $OPENSSL_LIB_DIR
echo $OPENSSL_INCLUDE_DIR
read
cargo build --manifest-path=native/Cargo.toml --target=aarch64-apple-ios
```

## 1.3. 工程配置

* 应用程序
  > cargo new rust-app
* 动态库
  > cargo new --lib rust-lib

</br>
</br>

# 2. Flutter 学习笔记

## 2.1. 环境配置

### 2.1.1. 安装配置 Flutter

* 地址：<https://flutter.cn/docs/get-started/install>
* 解压：$HOME/Dev/Flutter 或者 D:\Dev\Flutter
* 将解压路径添加到环境变量PATH中
* 查询版本：`$ flutter --version`
* 升级版本：`$ flutter upgrade`
* 检查配置：`$ flutter doctor`
* 检查位置：`$ where flutter dart`
* 安卓开发：<https://developer.android.google.cn/studio>

### 2.1.2. VSCode 插件

* 插件安装
  > Flutter 、Dart

### 2.1.3. 国内网络镜像

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

## 2.2. Flutter 平台相关

### 2.2.1. macOS 平台

* 需要 .zprofile 导出环境变量，否则 brew 更新困难
  > export HOMEBREW_NO_INSTALL_FROM_API=1
* macOS 与原生 iOS 代码交互
  > sudo gem install cocoapods
* macOS 需要通过AppStore安装 Xcode
* macOS 需要安装 clang
  > xcode-select --install

### 2.2.2. Android 平台

* 创建虚拟机图形加速选择：`Hardware - GLES 2.0`
* 可能需要创建环境变量：`export ANDROID_HOME="$HOME/.dev/android"`
* 安卓虚拟机 “关于” 页面连续点击 “Build number” 开启 “开发者模式”
* 卡在：Running Gradle task 'assembleDebug' 主要是GFW墙导致gradle下载缓慢，从国内镜像源下载<https://mirrors.cloud.tencent.com/gradle/>，之后拷贝到.gradle/wrapper/dists相应目录
* 注意用`flutter run -v`看缓慢原因

### 2.2.3. iOS 平台

* 启动模拟器：`open -a Simulator`

### 2.2.4. Windows 平台

* Android SDK Tools 真机调试需要安装 Google USB Driver

### 2.2.5. Linux 平台

* 可能缺失的依赖
  > which zip pkg-config clang cmake ninja libgtk-3-dev
* 可能需要添加软链接
  > sudo ln -s /usr/lib/llvm-14/lib/libclang.so.1 /usr/lib/llvm-14/lib/libclang.so

## 2.3. Rust FFI 连接

### 2.3.1. 安装配置

* 官网：<https://github.com/fzyzcjy/flutter_rust_bridge>
* 文档：<https://cjycode.com/flutter_rust_bridge/index.html>

* 工具和依赖

  > ```shell
  > dart pub global activate ffigen
  > cargo install flutter_rust_bridge_codegen

### 2.3.2. 用法

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

* 出入参数如果内含其他结构体，加上Option。

  ```rust
  pub fn query_report(query: ReportQuery) -> ReportPageDTO {
      APP.lock().unwrap().query_report(query)
  }
  pub struct ReportPageDTO {
      pub total: i32,
      pub list: Option<Vec<ReportDTO>>,
  }
  ```

## 2.4. 工程配置

* 创建应用程序
  > flutter create flutter_app
* 启动 iOS 模拟器
  > open -a Simulator
* 运行应用程序
  > flutter run
* 编译发布
  > flutter build windows
  > flutter build apk

## 2.5. Rust FFI 实战

* 创建Flutter应用
  > flutter create gitgui
* 进入gitgui目录，创建rust库
  > cargo new native --lib
* 修改native库配置 Cargo.toml

  ```toml
  [lib]
  crate-type = ["lib", "cdylib", "staticlib"]

  [dependencies]
  flutter_rust_bridge = "1"
  ```

* 安装转换工具
  > cargo install flutter_rust_bridge_codegen
  > cargo install cargo-expand
* 安装Flutter项目FFI依赖
  > flutter pub add --dev ffigen
  > flutter pub add ffi
* 配置Flutter项目
  * 连接桥Flutter端
    > flutter pub add flutter_rust_bridge
  * 生成平台绑定中使用的 Dart 代码
    > flutter pub add -d build_runner
  * 用于将对象从 Rust 传输到 Flutter
    > flutter pub add -d freezed
    > flutter pub add freezed_annotation
  * 生成平台绑定的代码
    > flutter_rust_bridge_codegen --rust-input native/src/api.rs --dart-output lib/bridge_generated.dart --dart-decl-output lib/bridge_definitions.dart
  * 获取Flutter依赖
    > flutter pub get
  * 查看Flutter过时包
    > flutter pub outdated
  * 更新Flutter依赖
    > flutter pub upgrade

## 2.6. FFI macOS & iOS 集成要点

* 配置cargo项目：`crate-type = ["lib", "cdylib", "staticlib"]`
* 生成xcode项目：`cargo xcode`
* 拖放添加macOS和iOS子项目，至根项目Runner下
* 设置根项目Runner的Target Runner，打开Build Phases页面，为**Target Dependencies**和**Link Binary with Libraries**添加依赖库，macOS选择**cdylib**库，iOS选择**staticlib**库
* 生成C头文件，参考justfile和build.rs，分别在macOS和iOS项目中引用，详见

```text
ios/Runner/Runner-Bridging-Header.h
ios/Runner/AppDelegate.swift
macos/Runner/AppDelegate.swift
```

* macOS手动添加C头文件：设置Target Runner，打开Build Settings页面，搜索**Objective-C Bridging Header**，双击添加生成的C库头文件**Runner/bridge_api.h**
* iOS防止strip：设置Target Runner，打开Build Settings页面，搜索**Strip Style**，修改成**Non-Global Symbols**
* iOS添加iconv链接依赖：设置Target Runner，打开Build Phases页面，为Link Binary with Libraries搜索**iconv**添加**libiconv.2.4.0.tbd**
* macOS找不到native.dylib：打开`cargo xcode`生成的native项目，设置Target native.cdylib，打开Build Settings页面，搜索**Dynamic Library Install Name Base**，修改成 **@executable_path/../Frameworks/** 。之后修改根项目Runner，设置Target Runner，打开Build Phases页面，为**Bundle Framework**添加**cdylib**将库添加到包中
* 分别设置根项目和native子项目，同步**PROJECT**的**Deployment Target**版本
* 分别设置根项目Target Runner和native子项目Target native-cdylib，打开Build Settings页面，修改**Build Active Architecture Only**为**Yes**，因为在x64平台下，默认无法编译arm64目标代码，如果不设置，在Release模式下，会出现openssl找不到（因为aarch64版本的openssl不存在）

## 2.7. Android & Gradle 国内镜像源配置

* 全局 ~/.gradle/init.gradle

```groovy
buildscript {
    repositories {
        maven { url 'https://maven.aliyun.com/repository/google' }
        maven { url 'https://maven.aliyun.com/repository/central' }
        maven { url 'https://maven.aliyun.com/repository/public' }
        google()
        mavenCentral()
    }

    allprojects {
        repositories {
            maven { url 'https://maven.aliyun.com/repository/google' }
            maven { url 'https://maven.aliyun.com/repository/central' }
            maven { url 'https://maven.aliyun.com/repository/public' }
            google()
            mavenCentral()
        }
    }
}
```

* 项目配置优先级高于全局：build.gradle，详见：<https://developer.aliyun.com/mvn/guide>

```groovy
allprojects {
  repositories {
    maven { url 'https://maven.aliyun.com/repository/public/' }
    maven { url 'https://maven.aliyun.com/repository/central' }
    mavenLocal()
    mavenCentral()
  }
}
```

* 新版本配置文件在：`settings.gradle`
* 官方推荐版本`settings.gradle.kts`

```groovy
pluginManagement {
    repositories {
        maven { url = uri("https://maven.aliyun.com/repository/central") }
        maven { url = uri("https://maven.aliyun.com/repository/jcenter") }
        maven { url = uri("https://maven.aliyun.com/repository/google") }
        maven { url = uri("https://maven.aliyun.com/repository/gradle-plugin") }
        maven { url = uri("https://maven.aliyun.com/repository/public") }
        maven { url = uri("https://jitpack.io") }
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        maven { url = uri("https://maven.aliyun.com/repository/central") }
        maven { url = uri("https://maven.aliyun.com/repository/jcenter") }
        maven { url = uri("https://maven.aliyun.com/repository/google") }
        maven { url = uri("https://maven.aliyun.com/repository/gradle-plugin") }
        maven { url = uri("https://maven.aliyun.com/repository/public") }
        maven { url = uri("https://jitpack.io") }
        google()
        mavenCentral()
    }
}
```

# 3. Tauri 学习笔记

## 3.1. 环境配置

### 3.1.1. 安装配置 Tauri

```shell
cargo install create-tauri-app --locked
cargo install tauri-cli
```

### 3.1.2. nodejs 转换淘宝源和官方源

```shell
npm config set registry https://registry.npm.taobao.org/
npm config set registry https://registry.npmjs.org/

yarn config set registry https://registry.npm.taobao.org/
yarn config set registry https://registry.npmjs.org/
```

### 3.1.3. 可能缺失的依赖

> webkit2gtk

## 3.2. 工程配置

* 创建工程
  > `cargo create-tauri-app`

* 运行工程
  > `cargo tauri dev`

* 发布工程
  > `cargo tauri build`

</br>
</br>
