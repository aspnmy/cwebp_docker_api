# cwebp API Service

## 1. 项目概述

cwebp API Service 是一个基于 Rust + Actix Web 开发的高性能图片转换服务，提供将各种格式图片转换为 WebP 格式的 API 接口。该服务集成了 cwebp 命令行工具，支持丰富的转换参数和多种响应类型。

## 2. 技术栈

* **Web框架**：Actix Web 4
* **图片处理**：集成 cwebp 命令行工具
* **文件上传**：actix-multipart
* **序列化**：serde
* **依赖管理**：Cargo
* **容器化**：Docker

## 3. 功能特性

* **支持多种图片格式**：PNG、JPEG、TIFF、WebP 等
* **支持无损和有损转换**
* **丰富的转换参数**：质量、压缩级别、预设配置等
* **两种响应类型**：
  * `webp`：返回 WebP 图片的下载地址（JSON 格式）
  * `base64`：返回 Base64 编码的 WebP 数据（JSON 格式）
* **健康检查端点**
* **文件自动清理**：支持通过 DELTIME 环境变量配置文件保留时间
* **图片大小限制**：支持通过 IMGSIZE 环境变量配置最大上传图片大小（MB）
* **Docker 容器化支持**：提供完整的 Dockerfile 和 docker-compose.yml
* **高性能、低内存占用**

## 4. 项目结构

```
cwebp_docker_api/
├── .github/
│   └── workflows/
│       └── docker-build.yml
├── src/                  # Rust + Actix Web 实现
│   ├── main.rs           # 主入口
│   ├── server.rs         # 服务器配置
│   ├── routes/           # API 路由
│   │   ├── convert.rs    # 图片转换路由
│   │   ├── health.rs     # 健康检查路由
│   │   └── mod.rs        # 路由模块
│   ├── services/         # 业务逻辑
│   │   └── cwebp.rs      # cwebp 命令封装
│   └── utils/            # 工具函数
│       └── file.rs       # 文件处理工具
├── Cargo.toml            # Rust 项目配置
├── Dockerfile            # Docker 配置
├── docker-compose.yml    # Docker Compose 配置
└── README.md             # 项目文档
```

## 5. 安装和运行

### 5.1 本地开发

#### 5.1.1 安装依赖

```bash
cargo build --release
```

#### 5.1.2 启动服务

```bash
cargo run --release
```

服务将在端口 3333 上运行。

### 5.2 Docker 部署

#### 5.2.1 构建 Docker 镜像

```bash
docker build -t cwebp-api .
```

#### 5.2.2 运行 Docker 容器

```bash
docker run -d -p 3333:3333 --name cwebp-api cwebp-api
```

#### 5.2.3 运行带配置的 Docker 容器

```bash
# 配置文件保留时间为 24 小时
docker run -d -p 3333:3333 -e DELTIME=24 --name cwebp-api cwebp-api

# 不自动删除文件
docker run -d -p 3333:3333 -e DELTIME=0 --name cwebp-api cwebp-api

# 配置图片大小限制为 50MB
docker run -d -p 3333:3333 -e IMGSIZE=50 --name cwebp-api cwebp-api

# 配置文件保留时间为 48 小时，图片大小限制为 200MB
docker run -d -p 3333:3333 -e DELTIME=48 -e IMGSIZE=200 --name cwebp-api cwebp-api
```

#### 5.2.4 容器间访问

在 Docker Compose 环境中，其他容器可以通过容器名称访问 cwebp API 服务：

```bash
curl -X POST -F "image=@input.jpg" http://cwebp-api:3333/api/convert
```

#### 5.2.5 使用 Docker Compose

```bash
# 启动服务
docker-compose up -d

# 重新构建并启动服务
docker-compose up -d --build

# 查看服务状态
docker-compose ps

# 查看服务日志
docker-compose logs -f

# 停止服务
docker-compose down

# 停止服务并删除数据卷
docker-compose down -v
```

## 6. API 文档

### 6.1 健康检查

* **URL**: `/health`
* **方法**: `GET`
* **功能**: 检查服务是否正常运行
* **响应**: JSON 格式的状态信息

### 6.2 图片转换

* **URL**: `/api/convert`
* **方法**: `POST`
* **功能**: 将图片转换为 WebP 格式
* **参数**: 
  * `image`: 要转换的图片文件（multipart/form-data）
  * `response_type`: 响应类型（webp/base64，默认 webp）
  * `lossless`: 是否使用无损转换（true/false，默认 false）
  * `quality`: 图片质量（0-100，默认 80）
  * `near_lossless`: 近无损转换级别（0-100，默认 100）
  * `compression_level`: 压缩级别（0-9，默认 6）
  * `preset`: 预设配置（default, photo, picture, drawing, icon, text）
  * `method`: 压缩方法（0-6，默认 4）

* **响应**: 
  * `webp` 类型：JSON 格式，包含图片 ID 和下载 URL
  * `base64` 类型：JSON 格式，包含 Base64 编码的 WebP 数据

### 6.3 获取 WebP 图片

* **URL**: `/api/images/:id`
* **方法**: `GET`
* **功能**: 获取转换后的 WebP 图片
* **参数**: 图片 ID（URL 路径参数）
* **响应**: WebP 图片二进制数据

## 7. 使用示例

### 7.1 健康检查

```bash
curl http://localhost:3333/health
```

### 7.2 上传图片并返回 webp 下载地址

```bash
curl -X POST -F "image=@input.jpg" http://localhost:3333/api/convert
```

### 7.3 上传图片并返回 base64 编码

```bash
curl -X POST -F "image=@input.jpg" -F "response_type=base64" http://localhost:3333/api/convert
```

### 7.4 获取 webp 图片

```bash
curl -X GET http://localhost:3333/api/images/1234567890abcdef -o output.webp
```

### 7.5 带转换参数的示例

```bash
# 无损转换，返回 base64
curl -X POST -F "image=@input.png" -F "lossless=true" -F "response_type=base64" http://localhost:3333/api/convert

# 有损转换，质量 75，压缩级别 9
curl -X POST -F "image=@input.jpg" -F "lossless=false" -F "quality=75" -F "compression_level=9" http://localhost:3333/api/convert
```

## 8. cwebp 命令说明

### 8.1 基本语法

```
cwebp [options] input_file -o output_file.webp
```

### 8.2 常用选项

* **无损压缩选项**：
  * `-lossless`：使用无损压缩
  * `-near_lossless <int>`：近无损压缩级别（0-100）
  * `-z <int>`：压缩级别（0-9）

* **有损压缩选项**：
  * `-q <float>`：图片质量（0-100）
  * `-m <int>`：压缩方法（0-6）

* **其他常用选项**：
  * `-preset <string>`：预设配置（default, photo, picture, drawing, icon, text）
  * `-alpha_q <int>`：Alpha 通道质量（0-100）
  * `-resize <width> <height>`：调整图片大小
  * `-crop <x> <y> <width> <height>`：裁剪图片
  * `-mt`：使用多线程编码

### 8.3 示例

* **示例 1**：无损压缩图片为 webp 格式（最高质量）
  ```
  cwebp -lossless -near_lossless 100 input.png -o output.webp
  ```

* **示例 2**：无损压缩图片为 webp 格式（中等质量）
  ```
  cwebp -lossless -near_lossless 60 input.png -o output.webp
  ```

* **示例 3**：有损压缩图片为 webp 格式（最高质量）
  ```
  cwebp -q 100 input.png -o output.webp
  ```

* **示例 4**：有损压缩图片为 webp 格式（中等质量）
  ```
  cwebp -q 75 input.png -o output.webp
  ```

## 9. 注意事项

1. **cwebp 工具**：服务依赖 cwebp 命令行工具，Docker 镜像已包含该工具
2. **文件存储**：转换后的文件保存在 `/app/img_webp` 目录
3. **文件自动清理**：
   * 支持通过 `DELTIME` 环境变量配置文件保留时间（小时）
   * 配置为 `0` 时，文件不会被自动删除
   * 配置为其他值时，超过该时间的文件会被自动清理
   * 清理任务每小时执行一次
4. **图片大小限制**：
   * 支持通过 `IMGSIZE` 环境变量配置最大上传图片大小（MB）
   * 默认值为 `100`（即 100MB）
   * 配置为 `0` 时，无大小限制
   * 超过限制的图片会被拒绝，返回 400 Bad Request 错误
5. **性能优化**：
   * 对于高并发场景，建议使用负载均衡
   * 可以调整 `method` 参数来平衡转换速度和文件大小
   * 对于频繁使用的转换参数，可以考虑使用预设配置
6. **错误处理**：服务会返回详细的错误信息，便于调试和问题定位
7. **Docker 部署**：
   * 建议使用 `docker-compose` 进行部署，简化配置和管理
   * 可以通过数据卷挂载 `/app/img_webp` 目录，实现数据持久化
8. **安全建议**：
   * 生产环境建议添加认证机制
   * 限制最大文件大小，防止过大的图片导致服务崩溃
   * 考虑添加 IP 白名单或访问控制

## 10. 项目结构详解

### 10.1 主入口文件（main.rs）

主入口文件负责初始化日志、启动服务器和处理全局配置。

### 10.2 服务器配置（server.rs）

服务器配置文件负责设置 Actix Web 服务器、注册路由和中间件。

### 10.3 路由模块（routes/）

* **convert.rs**：处理图片转换请求，包括文件上传、转换参数解析和响应生成
* **health.rs**：提供健康检查端点
* **mod.rs**：路由模块的入口文件，负责注册所有路由

### 10.4 业务逻辑（services/）

* **cwebp.rs**：封装 cwebp 命令行工具的调用，处理图片转换逻辑

### 10.5 工具函数（utils/）

* **file.rs**：提供文件处理相关的工具函数，包括临时文件管理和自动清理

## 11. 构建和部署

### 11.1 开发构建

```bash
cargo build
```

### 11.2 生产构建

```bash
cargo build --release
```

### 11.3 运行测试

```bash
cargo test
```

### 11.4 Docker 构建

```bash
docker build -t cwebp-api .
```

### 11.5 Docker Compose 示例

```yaml
version: '3.9'

services:
  cwebp-api:
    build: .
    image: cwebp-api:latest
    container_name: cwebp-api
    ports:
      - "3333:3333"
    environment:
      - DELTIME=72
    volumes:
      - ./img_webp:/app/img_webp
    restart: always
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3333/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

## 12. 性能特点

* **高性能**：基于 Rust 和 Actix Web，具有出色的并发处理能力
* **低内存占用**：Rust 的内存管理机制确保了高效的内存使用
* **快速启动**：服务启动时间短，适合容器化部署
* **高可靠性**：Rust 的类型系统和内存安全特性减少了运行时错误

## 13. 扩展建议

1. **添加认证机制**：为 API 添加 API 密钥认证或 OAuth 2.0 认证
2. **实现转换队列**：支持异步转换，处理大量并发请求
3. **添加监控和 metrics**：集成 Prometheus 和 Grafana，监控服务性能和转换统计
4. **支持更多转换参数**：扩展支持 cwebp 的所有命令行选项
5. **添加 Web UI**：提供简单易用的 Web 界面，便于测试和使用
6. **支持批量转换**：添加批量转换 API，一次处理多个图片
7. **实现图片大小限制**：防止过大的图片导致服务崩溃
8. **添加格式验证**：确保只处理支持的图片格式

## 14. 版本历史

* **v1.0.0 (2025-12-16)**：
  * 初始版本，基于 Rust + Actix Web 开发
  * 支持基本的图片转换功能
  * 支持两种响应类型：webp 和 base64
  * 提供 Docker 部署支持

## 15. 许可证

MIT License
