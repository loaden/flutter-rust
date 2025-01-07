
# 1. Tauri 学习笔记

记忆力逐渐丧失，只有笔记才能避免原地踏步。

## 1.1. 环境配置

### 1.1.1. 安装配置 Tauri

```shell
cargo install create-tauri-app --locked
cargo install tauri-cli
```

### 1.1.2. nodejs 转换淘宝源和官方源

```shell
npm config set registry https://registry.npm.taobao.org/
npm config set registry https://registry.npmjs.org/

yarn config set registry https://registry.npm.taobao.org/
yarn config set registry https://registry.npmjs.org/
```

### 1.1.3. 可能缺失的依赖

> webkit2gtk

## 1.2. 工程配置

* 创建工程
  > `cargo create-tauri-app`

* 运行工程
  > `cargo tauri dev`

* 发布工程
  > `cargo tauri build`
