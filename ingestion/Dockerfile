# 使用官方的 Rust 镜像作为构建阶段
FROM rust:latest AS builder

# 设置容器内的工作目录
WORKDIR /usr/src/ingestion

# 复制 Cargo.toml 和 Cargo.lock 文件
COPY Cargo.toml Cargo.lock ./

# 创建一个虚拟的 src/main.rs 文件以满足 cargo build 的要求
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# 先构建依赖项，以利用 Docker 层缓存
RUN cargo build --release
RUN rm -f src/*.rs

# 复制源代码
COPY . .

# 构建应用程序
RUN cargo build --release

# 使用最小的基础镜像
FROM ubuntu:latest

# 安装必要的运行时依赖
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq-dev \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# 设置容器内的工作目录
WORKDIR /usr/app

# 从构建阶段复制构建工件
COPY --from=builder /usr/src/ingestion/target/release/ingestion .

# 暴露应用程序运行的端口
EXPOSE 8000

# 运行应用程序
CMD ["./ingestion"]
