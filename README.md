# 🛠️ 开发工具箱

一个基于 Tauri + Vue 3 + TypeScript 构建的现代化开发工具集合，专为 Mac 平台优化。

## ✨ 功能特性

### 🔧 核心工具
- **JSON格式化** - 美化和压缩JSON数据
- **Base64编解码** - 文本与Base64之间的转换
- **URL编解码** - URL编码和解码工具
- **时间戳转换** - 时间戳与日期格式互转
- **正则表达式测试** - 实时测试正则表达式匹配
- **UUID生成器** - 快速生成UUID
- **哈希计算** - MD5和SHA256哈希值计算

### 🎨 界面特色
- 现代化的毛玻璃效果设计
- 响应式布局，支持不同屏幕尺寸
- 直观的侧边栏导航
- 一键复制功能
- 实时错误提示

## 🚀 快速开始

### 环境要求
- Node.js 16+
- Rust 1.70+
- pnpm

### 安装依赖
```bash
pnpm install
```

### 开发模式
```bash
pnpm tauri dev
```

### 构建应用
```bash
pnpm tauri build
```

## 📱 使用说明

1. **启动应用** - 运行开发命令后，应用会自动打开
2. **选择工具** - 点击左侧导航栏选择需要的工具
3. **输入数据** - 在左侧输入框中输入要处理的数据
4. **执行操作** - 点击相应的按钮执行转换或处理
5. **复制结果** - 点击复制按钮将结果复制到剪贴板

## 🛠️ 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Rust + Tauri
- **样式**: CSS3 (毛玻璃效果、渐变背景)
- **依赖管理**: pnpm

## 📦 项目结构

```
dev-tools/
├── src/                    # Vue前端源码
│   ├── App.vue            # 主应用组件
│   ├── main.ts            # 应用入口
│   └── assets/            # 静态资源
├── src-tauri/             # Tauri后端源码
│   ├── src/
│   │   ├── lib.rs         # 核心功能实现
│   │   └── main.rs        # 应用入口
│   ├── Cargo.toml         # Rust依赖配置
│   └── tauri.conf.json    # Tauri配置
├── package.json           # Node.js依赖配置
└── README.md             # 项目说明
```

## 🔧 开发指南

### 添加新工具
1. 在 `src-tauri/src/lib.rs` 中添加Rust函数
2. 在 `src/App.vue` 中添加对应的Vue组件
3. 更新工具列表和导航

### 自定义样式
- 主要样式在 `src/App.vue` 的 `<style>` 部分
- 支持响应式设计和深色模式

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).
