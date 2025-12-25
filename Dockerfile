# 多阶段构建 Dockerfile for Tron Address Generator

# 第一阶段：构建阶段
FROM rust:1.83-slim as builder

# 设置工作目录
WORKDIR /app

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./

# 复制源代码
COPY src ./src

# 构建发布版本
RUN cargo build --release

# 第二阶段：运行阶段
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -m -u 1000 tron && \
    mkdir -p /app/output && \
    chown -R tron:tron /app

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/tron_address /usr/local/bin/tron_address

# 切换到非 root 用户
USER tron

# 设置环境变量
ENV MIN_SAME_CHARS=6

# 设置输出目录为卷
VOLUME ["/app/output"]

# 默认命令
ENTRYPOINT ["tron_address"]
CMD ["--output", "/app/output/addresses.txt"]

