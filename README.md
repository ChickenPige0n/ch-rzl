# ch-rzl

## 该项目是一个通过h5的canvasAPI制作的还原游戏《Rizline》的个人兴趣项目。

## 该项目于鸽游无关

## Rust 版本 (Bevy/wgpu)

本项目现在包含一个使用 Rust + Bevy + wgpu 实现的前端播放器。

### 系统要求

- Rust 1.70+ (推荐使用 rustup 安装)
- 支持 Vulkan、Metal 或 DirectX 12 的显卡

### Linux 依赖

在 Ubuntu/Debian 上安装必要的依赖：

```bash
sudo apt-get update
sudo apt-get install -y libasound2-dev libudev-dev pkg-config
```

### 构建

```bash
# 开发版本
cargo build

# 发布版本（优化后）
cargo build --release
```

### 运行

```bash
# 运行默认谱面
cargo run

# 运行指定谱面文件
cargo run -- path/to/chart.json
```

### 控制

- **空格键** - 播放/暂停
- **R** - 重置到开始
- **左/右方向键** - 前进/后退
- **上/下方向键** - 调整速度

### 项目结构

```
src/
├── main.rs       # 程序入口
├── chart.rs      # 谱面数据结构
├── easing.rs     # 缓动函数 (19种)
├── timing.rs     # 时间/节拍转换
├── game.rs       # 游戏状态和逻辑
└── rendering.rs  # Bevy 渲染系统
```

---

## 原始 JavaScript 版本

原始的 JavaScript 版本仍然可用，打开 `index.html` 即可在浏览器中运行。

## QQ：1095216488