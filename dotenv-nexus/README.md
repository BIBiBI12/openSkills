# .env Nexus

**The modern .env file manager for developers**

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-brightgreen.svg)
![Language](https://img.shields.io/badge/language-Rust-orange.svg)

Never commit `.env` files again, but never lose them either.

## 🔴 The Problem

- You have multiple projects with multiple `.env` files
- You **can't** commit them to git (they contain secrets!)
- You forget where which variable came from
- You need to share non-secret configs with team members
- You have different environments (`.env`, `.env.local`, `.env.development`, `.env.production` chaos)
- You lose track of what changed between environments

## 🟢 The Solution

**.env Nexus** is a CLI tool that:

- 🔒 **Encrypts** and stores your `.env` files **securely** in git
- 👥 Lets you share secrets with team members safely
- 🌲 Manages dev/staging/prod environments in one place
- 🔄 Works with your existing `git` workflow
- ⚡ **Zero config** - just run it
- 📝 Automatically generates `.env.example` templates
- 🔍 Show differences between environments

## 📦 Installation

```bash
cargo install dotenv-nexus
```

Or build from source:

```bash
git clone https://github.com/BIBiBI12/openSkills.git
cd openSkills/dotenv-nexus
cargo install --path .
```

## 🚀 Quick Start

```bash
# 1. Initialize in your existing project
dnx init

# 2. Add your current .env file
dnx add .env --env development

# 3. Encrypt all environments
dnx encrypt
# (you'll be prompted for a password)

# 4. Commit to git safely
git add .
git commit -m "Add encrypted environment configs"
git push

# 5. When you clone the repo again
dnx decrypt
# (enter the same password)

# 6. Switch between environments
dnx switch development
dnx switch production
```

## 💡 How It Works

1. `dnx init` creates a `.env-nexus` directory in your project
2. `dnx add` encrypts your `.env` files with AES-256-GCM
3. Encrypted files are stored in `.env-nexus/` directory
4. You **can** commit `.env-nexus` to git (it's safe!)
5. Only people with the decryption password can recover original files
6. `dnx encrypt` removes plaintext `.env` files, leaving only encrypted ones
7. `dnx decrypt` restores plaintext files when needed

## 📋 Features

| Feature | Description |
|---------|-------------|
| 🔒 Encryption | AES-256-GCM encryption with password |
| 👥 Team Sharing | Share encrypted configs via git |
| 🌲 Environments | Manage dev/staging/prod in one place |
| 🔄 Git-Compatible | Stores encrypted env alongside your code |
| ⚡ Zero Config | Works out of the box with existing projects |
| 📝 Auto Template | Generates `.env.example` automatically |
| 🔍 Diff Support | See what changed between environments |
| 💻 Cross-Platform | Linux, macOS, Windows |

## 🎯 Use Cases

- **Personal projects** - keep your env files safe in git
- **Small teams** - share secrets without expensive enterprise tools
- **Open source** - public repo with private configuration
- **DevOps** - manage different environments easily
- **Disaster recovery** - never lose your .env files again

## 💻 Commands

```bash
dnx init                          # Initialize in current directory
dnx add .env --env development     # Add environment file
dnx encrypt                        # Encrypt all environments
dnx decrypt                        # Decrypt all environments
dnx list                           # List all configured environments
dnx switch production               # Switch to environment
dnx diff [env1] [env2]            # Show differences
```

## 📁 Project Structure

```
your-project/
├── .env                    # Current environment (dynamically generated)
├── .env.example             # Template for new developers
├── .gitignore               # .env should be in here
├── .env-nexus/             # Encrypted environments (safe to commit!)
│   ├── config.json           # Configuration
│   ├── development.env       # Development environment (encrypted)
│   ├── staging.env          # Staging environment (encrypted)
│   └── production.env        # Production environment (encrypted)
└── src/                    # Your source code
```

## 🔐 Security

- Uses **AES-256-GCM** encryption
- Password-derived key with SHA-256
- **NEVER stores plaintext passwords** (you enter it each time)
- Encrypted files are safe to commit to git
- Only people with the password can decrypt

## ⚠️ Why Not Other Approaches?

| Approach | Secrets Safe | Versioned | Shareable | Git-Only |
|----------|--------------|-----------|-----------|-----------|
| `.env` in `.gitignore` | ✅ | ❌ | ❌ | ✅ |
| `.env.example` only | ✅ | ✅ | ❌ (secrets) | ✅ |
| Secret managers (AWS, etc.) | ✅ | ✅ | ✅ | ❌ (external service) |
| **.env Nexus** | ✅ | ✅ | ✅ | ✅ (everything in git) |

## 🤝 Contributing

Issues and PRs are welcome! 

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details

## 🙏 Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - Command line parsing
- [aes-gcm](https://github.com/RustCrypto/AEADs) - Encryption
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling
- [dialoguer](https://github.com/mitsuhiko/dialoguer) - Interactive prompts

## 📞 Support

- 🐛 Report bugs: [GitHub Issues](https://github.com/BIBiBI12/openSkills/issues)
- 💬 Ask questions: [GitHub Discussions](https://github.com/BIBiBI12/openSkills/discussions)
- 📧 Email: [Issues only - no email support]

---

**Made with ❤️ for developers who hate managing .env files**
