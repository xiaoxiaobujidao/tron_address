# Tron Address Generator 🚀

高性能的 Tron（波场）靓号地址生成器，使用 Rust 编写，支持多核并行计算。可以生成末尾具有相同字符的特殊 Tron 地址。

## ✨ 特性

- 🚀 **高性能**: 使用 Rust 编写，利用 Rayon 实现多核并行计算
- 🎯 **定制化**: 可指定末尾相同字符的最小数量
- 📊 **实时统计**: 显示生成速度和进度
- 💾 **批量输出**: 自动保存找到的地址和私钥
- 🐳 **Docker 支持**: 提供 Docker 镜像，开箱即用
- 🔒 **安全性**: 使用标准的加密库生成密钥

## 📦 Docker 使用方法

### 拉取镜像

从 GitHub Container Registry 拉取最新镜像：

```bash
docker pull ghcr.io/xiaoxiaobujidao/tron_address:latest
```

### 基本使用

生成末尾至少 6 个相同字符的地址：

```bash
docker run -v $(pwd)/output:/app/output ghcr.io/xiaoxiaobujidao/tron_address:latest
```

### 自定义参数

指定末尾至少 8 个相同字符：

```bash
docker run -v $(pwd)/output:/app/output \
  ghcr.io/xiaoxiaobujidao/tron_address:latest \
  --min-same-chars 8 \
  --output /app/output/addresses.txt
```

限制生成 10 个地址后停止：

```bash
docker run -v $(pwd)/output:/app/output \
  ghcr.io/xiaoxiaobujidao/tron_address:latest \
  --min-same-chars 6 \
  --limit 10 \
  --output /app/output/addresses.txt
```

指定使用的 CPU 核心数：

```bash
docker run -v $(pwd)/output:/app/output \
  ghcr.io/xiaoxiaobujidao/tron_address:latest \
  --min-same-chars 7 \
  --cores 8 \
  --output /app/output/addresses.txt
```

### 后台运行

```bash
docker run -d \
  --name tron-generator \
  -v $(pwd)/output:/app/output \
  ghcr.io/xiaoxiaobujidao/tron_address:latest \
  --min-same-chars 7 \
  --limit 100
```

查看日志：

```bash
docker logs -f tron-generator
```

停止容器：

```bash
docker stop tron-generator
docker rm tron-generator
```

## 🛠️ 本地编译使用

### 前置要求

- Rust 1.70 或更高版本
- Cargo

### 编译

```bash
cargo build --release
```

### 运行

```bash
# 使用默认参数（末尾至少 6 个相同字符）
./target/release/tron_address

# 自定义参数
./target/release/tron_address \
  --min-same-chars 7 \
  --cores 8 \
  --output my_addresses.txt \
  --limit 50 \
  --batch-size 100000
```

## 📋 命令行参数

| 参数 | 短参数 | 默认值 | 说明 |
|------|--------|--------|------|
| `--min-same-chars` | `-m` | 6 | 末尾相同字符的最小数量 |
| `--cores` | `-c` | CPU核心数 | 使用的 CPU 核心数 |
| `--output` | `-o` | output | 输出文件名 |
| `--limit` | `-l` | 0（无限制） | 生成地址数量限制 |
| `--batch-size` | `-b` | 50000 | 批处理大小 |

## 📝 输出格式

生成的地址将保存在指定的输出文件中，格式如下：

```
地址: TYourTronAddressHere888888
私钥: your_private_key_in_hex_format
相同字符数: 6
---
地址: TAnotherTronAddress777777
私钥: another_private_key_in_hex
相同字符数: 6
---
```

## 🌟 示例地址

以下是可能生成的靓号地址示例：

- `TXxxxxxxxxxxxxxxxxxxxx888888` - 末尾 6 个 8
- `TXxxxxxxxxxxxxxxxxxxxx777777` - 末尾 6 个 7
- `TXxxxxxxxxxxxxxxxxxxxx666666` - 末尾 6 个 6

## ⚠️ 安全警告

1. **私钥安全**: 生成的私钥具有完全的资金控制权，请妥善保管输出文件
2. **文件权限**: 建议设置输出文件的权限为仅所有者可读（`chmod 600`）
3. **不要分享私钥**: 永远不要将私钥分享给任何人或上传到公共位置
4. **测试使用**: 建议先用少量资金测试生成的地址

## 📊 性能参考

性能取决于 CPU 性能和相同字符数量要求：

- 6 个相同字符：约 1000-5000 地址/秒（具体取决于 CPU）
- 7 个相同字符：约 100-500 地址/秒
- 8 个相同字符：约 10-50 地址/秒
- 9+ 个相同字符：需要更长时间

## 🔧 环境变量

可以通过环境变量设置默认值：

```bash
# 设置默认最小相同字符数
export MIN_SAME_CHARS=7

# Docker 中使用环境变量
docker run -e MIN_SAME_CHARS=7 \
  -v $(pwd)/output:/app/output \
  ghcr.io/xiaoxiaobujidao/tron_address:latest
```

## 🐛 故障排除

### 容器无法写入文件

确保输出目录有正确的权限：

```bash
mkdir -p output
chmod 777 output  # 或者使用更安全的权限设置
```

### CPU 使用率低

尝试增加批处理大小：

```bash
docker run -v $(pwd)/output:/app/output \
  ghcr.io/xiaoxiaobujidao/tron_address:latest \
  --batch-size 100000
```

### 找不到镜像

确保已登录 GitHub Container Registry：

```bash
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin
```

## 📄 许可证

本项目使用 MIT 许可证。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## ⚡ 更新日志

### v0.1.0
- 初始版本
- 支持多核并行生成
- 支持自定义相同字符数量
- Docker 支持
- GitHub Actions 自动构建

## 📞 联系方式

如有问题或建议，请提交 [Issue](../../issues)。

---

**注意**: 本工具仅供学习和研究使用，请遵守当地法律法规。生成的地址和私钥的安全性由用户自行负责。

