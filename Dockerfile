# 多阶段构建 - 第一阶段：构建
FROM rust:1.83-slim as builder

WORKDIR /app

# 安装必要的依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# 复制依赖文件
COPY Cargo.toml Cargo.lock ./

# 创建一个虚拟的 main.rs 来缓存依赖
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# 复制实际的源代码
COPY src ./src

# 构建应用（此时依赖已缓存，只需要构建应用代码）
RUN touch src/main.rs && \
    cargo build --release

# 第二阶段：运行
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 从构建阶段复制编译好的二进制文件
COPY --from=builder /app/target/release/tron_address /usr/local/bin/tron_address

# 创建输出目录
RUN mkdir -p /app/output

# 设置工作目录
WORKDIR /app/output

# 设置入口点
ENTRYPOINT ["tron_address"]

# 默认参数（可以在运行时覆盖）
CMD ["--help"]

