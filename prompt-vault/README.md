# 🗄️ Prompt Vault

> 你的个人提示词保险库 - 结构化管理所有AI提示词，永不丢失，一键获取。

## 为什么需要 Prompt Vault？

你是否遇到过这些问题：
- 💢 收藏了上百个ChatGPT提示词，要用的时候死活找不到？
- 💢 同一个提示词改了十几个版本，不知道哪个是最新的？
- 💢 换电脑/重装系统，所有提示词都没了？
- 💢 想要分享给朋友，却是零散的截图和文本？

**Prompt Vault 解决了所有这些问题！**

## 功能特点

✅ **结构化组织** - 分类 + 标签，多维度筛选搜索  
✅ **版本控制** - 记录每个提示词的演进历史  
✅ **一键复制** - 终端快捷操作，复制即用不麻烦  
✅ **导入导出** - 轻松备份迁移，也能分享他人  
✅ **隐私优先** - 完全本地运行，你的提示词只属于你  
✅ **轻量快速** - 单个二进制文件，无依赖，启动秒开  
✅ **美观界面** - 终端交互式UI，一目了然  

## 快速开始

### 安装

```bash
# 下载二进制文件 (从 Releases 页面)
wget https://github.com/你的用户名/prompt-vault/releases/download/v0.1.0/prompt-vault_0.1.0_linux_amd64.tar.gz
tar -xzf prompt-vault_0.1.0_linux_amd64.tar.gz
mv prompt-vault /usr/local/bin/
```

### 基本使用

```bash
# 初始化仓库
prompt-vault init ~/.prompt-vault

# 添加一个提示词
prompt-vault add

# 搜索提示词
prompt-vault search "coding"

# 启动交互式终端UI
prompt-vault tui

# 导出为JSON分享
prompt-vault export prompts.json
```

## 项目结构

```
~/.prompt-vault/
├── prompts/
│   ├── category1/
│   │   ├── prompt1.md
│   │   └── prompt2.md
│   └── category2/
│       └── prompt3.md
├── index.db
└── config.yaml
```

## 支持的功能

- [x] 基础CRUD操作
- [x] 分类和标签系统
- [x] 全文搜索
- [x] 终端TUI界面
- [ ] Web界面
- [ ] 云端同步(可选)
- [ ] 社区提示词分享市场
- [ ] VS Code 插件

## 为什么选择 Prompt Vault？

| 特性 | Prompt Vault | 笔记软件 | 浏览器书签 |
|------|--------------|----------|------------|
| 专为提示词优化 | ✅ | ❌ | ❌ |
| 本地隐私 | ✅ | ✅ | ❌ |
| 快速搜索 | ✅ | ⚠️ | ❌ |
| 版本历史 | ✅ | ⚠️ | ❌ |
| 一键复制使用 | ✅ | ❌ | ❌ |
| 完全免费开源 | ✅ | - | - |

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License - 见 [LICENSE](LICENSE) 文件

---

如果你觉得这个项目有用，请给我一个 ⭐ Star 支持一下！谢谢！
