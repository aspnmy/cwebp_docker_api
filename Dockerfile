FROM docker.io/library/debian:13.2-slim AS builder
# 设置工作目录
WORKDIR /app
# 安装依赖
RUN apt-get update --allow-releaseinfo-change && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    wget \
    && wget https://storage.googleapis.com/downloads.webmproject.org/releases/webp/libwebp-1.6.0-linux-x86-64.tar.gz \
    && tar -xzvf libwebp-1.6.0-linux-x86-64.tar.gz \
    && mkdir -p /app/libwebp \
    && cp libwebp-1.6.0-linux-x86-64/bin/* /app/libwebp \
    && chmod +x /app/libwebp/* \
    && rm -rf libwebp-1.6.0-linux-x86-64* \
    && rm -rf /var/lib/apt/lists/*
# 降低镜像大小
FROM docker.io/library/debian:13.2-slim
# 设置工作目录
WORKDIR /app
# 复制编译好的文件
COPY --from=builder /app/libwebp /app/libwebp
RUN chmod +x /app/libwebp/*
# 暴露端口
EXPOSE 3333
#EXPOSE 80

# 运行启动脚本
CMD ["./libwebp/cwebp"]
