<p align="center"><img src="/src/assets/SRAico.png" alt="icon"></p>
<p align="center">
    <img src="https://img.shields.io/badge/platform-Windows-blue" alt="platform">
    <img alt="Static Badge" src="https://img.shields.io/badge/python-3.12-skyblue">
    <img alt="Static Badge" src="https://img.shields.io/badge/Tauri-2.0-24c8db">
    <img alt="Static Badge" src="https://img.shields.io/badge/Vue-3.0-42b883">
    <img alt="GitHub Release" src="https://img.shields.io/github/v/release/EveGlowLuna/StarRailAssistant-CommunityEdition">
</p>

# StarRailAssistant Community Edition (SRA-CE)

崩坏星穹铁道自动化助手 - 社区版

## 项目简介

这是基于 [StarRailAssistant](https://github.com/Shasnow/StarRailAssistant) 的社区版本，使用 **Tauri + Vue 3** 重构了前端界面，提供更现代化的用户体验。

### 与原版的区别

- ✨ **全新前端**：使用 Tauri + Vue 3 + TypeScript 重构，界面更美观、性能更优
- 🚀 **更小体积**：相比原版 .NET 前端，打包体积更小、无需安装.NET Runtime
- 🎨 **现代化 UI**：采用现代化的设计语言和交互方式
- 🔧 **易于扩展**：基于 Web 技术栈，更容易进行二次开发和定制

## 什么是 SRA？

一个基于图像识别的崩铁自动化程序，帮您完成从启动到退出的崩铁日常。

## 免责声明

本软件是一个外部工具旨在自动化 崩坏：星穹铁道 的游戏玩法。它被设计成仅通过现有用户界面与游戏交互,并遵守相关法律法规。该软件包不打算以任何方式破坏游戏平衡或提供任何不公平的优势。该软件包不会以任何方式修改任何游戏文件或游戏代码。

This software is open source, free of charge and for learning and exchange purposes only. The developer team has the final right to interpret this project. All problems arising from the use of this software are not related to this project and the developer team. If you encounter a merchant using this software to practice on your behalf and charging for it, it may be the cost of equipment and time, etc. The problems and consequences arising from this software have nothing to do with it.

本软件开源、免费，仅供学习交流使用。开发者团队拥有本项目的最终解释权。使用本软件产生的所有问题与本项目与开发者团队无关。若您遇到商家使用本软件进行代练并收费，可能是设备与时间等费用，产生的问题及后果与本软件无关。

请注意，根据 MiHoYo 的 [崩坏:星穹铁道的公平游戏宣言](https://sr.mihoyo.com/news/111246?nav=news&type=notice):

    "严禁使用外挂、加速器、脚本或其他破坏游戏公平性的第三方工具。"
    "一经发现，米哈游（下亦称"我们"）将视违规严重程度及违规次数，采取扣除违规收益、冻结游戏账号、永久封禁游戏账号等措施。"

## 功能特性

* **启动游戏**
    * 选择游戏路径，输入账号与密码，程序会帮你解决好一切。已适配 B 服。
* **清体力**
    * 自由选择关卡，设置是否补充体力、连战次数、执行次数，支持混合搭配。
* **领取奖励**
    * 包括：每日实训、无名勋礼、助战奖励、兑换码奖励、派遣奖励、巡星之礼、邮件奖励等。
* **差分宇宙刷等级**
* **货币战争**
* **退出游戏**
    * 可选退出 SRA 以及关闭计算机。
* **自动剧情**
    * 解放双手，享受剧情，支持跳过。
* **多账号切换**

## 快速开始

### 下载安装

前往 [Releases](https://github.com/EveGlowLuna/StarRailAssistant-CommunityEdition/releases/latest)或 [AList](https://alist.starrailassistant.xyz/) 页面下载最新版本：

- **安装包版本**：`StarRailAssistant-{版本号}-setup.exe` - 适合需要安装到系统的用户
- **便携版本**：`StarRailAssistant-{版本号}-portable.zip` - 解压即用，适合便携使用

### 使用方法

1. 下载并安装/解压程序
2. 运行 `StarRailAssistant.exe`
3. 配置游戏路径和账号信息
4. 选择需要执行的任务
5. 点击开始，坐和放宽

### 注意事项

* **调整游戏分辨率为 1920×1080（推荐）并保持游戏窗口无遮挡，不要让游戏窗口超出屏幕**
* **执行任务时不要进行其他键鼠操作！**
* **菜单要用初始壁纸**

## 从源码构建

### 环境要求

- Node.js 18+
- Python 3.12+
- Rust (Tauri 依赖)

### 安装依赖

```bash
# 安装前端依赖
npm install

# 安装后端依赖
cd Backend
pip install -r requirements.txt
cd ..
```

### 开发模式

```bash
# 后端
python main.py
# 启动开发服务器
# 提前打包 Python 文件以进行全面调试
python package.py
npm run tauri dev
```

### 构建发布版本

```bash
# 打包后端
cd Backend
python package.py
cd ..

# 打包主程序
npm run package
```

构建完成后，在 `src-setup/output` 目录下可以找到：
- 安装包：`StarRailAssistant-{版本号}-setup.exe`
- 便携版：`StarRailAssistant-{版本号}-portable.zip`

## 技术栈

### 前端
- Tauri 2.0 - 跨平台桌面应用框架
- Vue 3 - 渐进式 JavaScript 框架
- TypeScript - 类型安全的 JavaScript 超集
- Vite - 下一代前端构建工具

### 后端
- Python 3.12 - 核心逻辑实现
- OpenCV - 图像识别
- Nuitka - Python 打包工具

## 反馈与支持

如果您在使用过程中遇到问题或有建议，欢迎通过以下方式反馈：

* 通过 **Issues** 反馈：[提交 Issue](https://github.com/EveGlowLuna/StarRailAssistant-CommunityEdition/issues)
* 通过 **Pull Request** 贡献代码

## 贡献指南

欢迎为项目做出贡献！如果你希望参与开发：

### 后端开发
* 熟悉 Python
* 正在游玩并将长期游玩崩坏：星穹铁道

### 前端开发
* 熟悉 Vue 3 和 TypeScript
* 了解 Tauri 框架

## 致谢

本项目基于 [StarRailAssistant](https://github.com/Shasnow/StarRailAssistant) 开发，感谢原作者 Shasnow 及所有贡献者的付出。

## 开源协议

本项目采用 GPL-3.0 协议开源，详见 [LICENSE](LICENSE) 文件。

---

**注意**：本项目仅供学习交流使用，请勿用于商业用途。使用本软件产生的一切后果由使用者自行承担。
