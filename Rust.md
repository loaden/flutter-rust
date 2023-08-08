# Rust 学习笔记

记忆力逐渐丧失，只有笔记才能避免原地踏步。

## 一、基础知识

### 1. 创建项目

* 应用程序
  > cargo new rust-app
* 动态库
  > cargo new --lib rust-lib

### 2. VSCode

* 插件安装
  > rust-analyzer

### 3. 下载加速

* .cargo/config.toml

    ```toml
  [source.crates-io]
  replace-with = 'ustc'

  [source.ustc]
  registry = "git://mirrors.ustc.edu.cn/crates.io-index/"

    ```