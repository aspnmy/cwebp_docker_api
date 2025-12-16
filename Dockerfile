FROM rust:alpine3.23 AS builder

# 设置工作目录
WORKDIR /app

# 安装必要的构建依赖
RUN apk add --no-cache musl-dev gcc make wget

# 复制Cargo.toml和Cargo.lock
COPY Cargo.toml Cargo.lock ./

# 创建一个空的main.rs文件，用于构建依赖
RUN mkdir -p src && echo 'fn main() {}' > src/main.rs

# 构建依赖
RUN cargo build --release


# 复制源代码
COPY src ./src

# 构建应用
RUN cargo build --release

# 下载cwebp预构建版本
RUN wget https://storage.googleapis.com/downloads.webmproject.org/releases/webp/libwebp-1.6.0-linux-x86-64.tar.gz \
    && tar -xzvf libwebp-1.6.0-linux-x86-64.tar.gz \
    && mkdir -p /app/libcweb \
    && cp -r libwebp-1.6.0-linux-x86-64/* /app/libcweb/ 


# 最终版本
FROM alpine:3.23.0

# 设置工作目录
WORKDIR /app


# 创建输出目录
RUN mkdir -p /app/img_webp \
    && chmod 777 /app/img_webp \
    && chown -R 1000:1000 /app/img_webp \
    && mkdir -p /app/cwebp_rustapi \
    && chmod 777 /app/cwebp_rustapi \
    && chown -R 1000:1000 /app/cwebp_rustapi \
    && mkdir -p /app/libcwebp \
    && chmod 777 /app/libcwebp \
    && chown -R 1000:1000 /app/libcwebp

# 复制cwebp工具和库
COPY --from=builder /app/libcweb/* /app/libcwebp

# 复制构建好的应用
COPY --from=builder /app/target/release/cwebp_docker_api /app/cwebp_rustapi

# 确保cwebp工具具有执行权限
RUN chmod +x /app/libcwebp/cwebp && chmod +x /app/cwebp_rustapi/cwebp_docker_api

# 设置环境变量
ENV DELTIME=72
ENV IMGSIZE=100
ENV PATH="/app/libcwebp:${PATH}"
ENV LD_LIBRARY_PATH="/app/libcwebp:${LD_LIBRARY_PATH}"

# 暴露端口
EXPOSE 3333

# 运行应用
CMD ["./cwebp_docker_api"]
