# WSL2部署Podman容器打包环境

本指南详细介绍了如何在WSL2环境中部署Podman、Docker、Docker Compose等容器工具，以及Git、Rust、Cargo等开发工具，用于项目代码编译、测试、Podman容器构建与调试，最终用于GitHub私有化Runner。

## 1. WSL2环境准备

### 1.1 安装WSL2

在Windows 10/11上安装WSL2，请参考Microsoft官方文档：

```powershell
# 以管理员身份运行PowerShell
wsl --install
```

### 1.2 配置WSL2

安装完成后，打开WSL2终端，更新系统：

```bash
sudo apt update && sudo apt upgrade -y
```

### 1.3 安装基本依赖

```bash
sudo apt install -y build-essential git curl wget unzip jq
```

## 2. 容器工具安装

### 2.1 安装Podman

```bash
# 添加Podman仓库
source /etc/os-release
echo "deb https://download.opensuse.org/repositories/devel:/kubic:/libcontainers:/unstable/xUbuntu_${VERSION_ID}/ /" | sudo tee /etc/apt/sources.list.d/devel:kubic:libcontainers:unstable.list
curl -fsSL https://download.opensuse.org/repositories/devel:/kubic:/libcontainers:/unstable/xUbuntu_${VERSION_ID}/Release.key | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/libcontainers.gpg

# 安装Podman
sudo apt update && sudo apt install -y podman

# 验证安装
podman --version
```

### 2.2 配置Podman

```bash
# 启用无根Podman
systemctl --user enable --now podman.socket

# 配置Docker兼容别名
echo "alias docker='podman'" >> ~/.bashrc
echo "alias docker-compose='podman-compose'" >> ~/.bashrc
source ~/.bashrc

# 验证无根配置
podman info | grep -i rootless
```

### 2.3 安装Podman Compose

```bash
# 方式1：使用pip安装
sudo apt install -y python3-pip
pip3 install podman-compose

# 方式2：使用apt安装（如果可用）
# sudo apt install -y podman-compose

# 验证安装
podman-compose --version
```

### 2.4 安装Docker Engine（可选）

如果需要同时使用Docker Engine，可以按照以下步骤安装：

```bash
# 添加Docker仓库
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# 安装Docker Engine
sudo apt update && sudo apt install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin

# 启动Docker服务
sudo systemctl enable --now docker

# 将当前用户添加到docker组
sudo usermod -aG docker $USER

# 重新登录后验证安装
docker --version
docker-compose --version
```

## 3. 开发工具安装

### 3.1 安装Git

```bash
# 安装Git
sudo apt install -y git

# 配置Git
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# 验证安装
git --version
```

### 3.2 安装Rust和Cargo

```bash
# 使用rustup安装Rust和Cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 按照提示完成安装，选择默认选项即可

# 配置环境变量
source $HOME/.cargo/env

# 验证安装
rustc --version
cargo --version
```

### 3.3 安装其他开发工具

```bash
# 安装编译工具链
sudo apt install -y gcc g++ make cmake

# 安装Python（可选）
sudo apt install -y python3 python3-pip python3-venv

# 安装Node.js（可选）
curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash -
sudo apt install -y nodejs
```

## 4. Podman仓库配置

### 4.1 配置本地镜像仓库

```bash
# 创建本地仓库目录
sudo mkdir -p /var/lib/containers/registry

# 运行本地镜像仓库
podman run -d \
  --name registry \
  -p 5000:5000 \
  -v /var/lib/containers/registry:/var/lib/registry \
  --restart=always \
  docker.io/library/registry:2

# 验证本地仓库
echo "FROM docker.io/library/alpine:latest" > Dockerfile
podman build -t localhost:5000/test:latest .
podman push localhost:5000/test:latest
```

### 4.2 配置远程镜像仓库

```bash
# 登录到GitHub Container Registry
podman login ghcr.io

# 登录到Docker Hub
podman login docker.io

# 登录到私有镜像仓库
podman login --username <username> --password <password> <registry-url>
```

## 5. 代码编译和测试环境

### 5.1 配置Rust开发环境

```bash
# 安装Rust工具链组件
rustup component add clippy rustfmt rls rust-analysis

# 安装cargo工具
cargo install cargo-audit cargo-tree cargo-outdated

# 验证配置
cargo clippy --version
cargo fmt --version
```

### 5.2 编译示例Rust项目

```bash
# 创建测试项目
cargo new test-project
cd test-project

# 编译项目
cargo build

# 运行测试
cargo test

# 构建发布版本
cargo build --release
```

## 6. Podman容器构建与调试

### 6.1 构建Docker镜像

```bash
# 创建测试Dockerfile
cat > Dockerfile << EOF
FROM docker.io/library/rust:1.75.0-alpine3.19 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM docker.io/library/alpine:3.19
WORKDIR /app
COPY --from=builder /app/target/release/test-project .
CMD ["./test-project"]
EOF

# 使用Podman构建镜像
podman build -t test-rust-app .

# 查看构建的镜像
podman images
```

### 6.2 运行和调试容器

```bash
# 运行容器
podman run -d --name test-rust-container test-rust-app

# 查看容器日志
podman logs -f test-rust-container

# 进入容器调试
podman exec -it test-rust-container /bin/sh

# 查看容器资源使用情况
podman stats test-rust-container
```

### 6.3 容器网络配置

```bash
# 创建自定义网络
podman network create test-network

# 在自定义网络中运行容器
podman run -d --name test-container --network test-network test-rust-app

# 查看网络配置
podman network inspect test-network
```

## 7. GitHub私有化Runner配置

### 7.1 安装Runner依赖

```bash
# 安装Runner依赖
sudo apt install -y libssl-dev libffi-dev python3-dev default-jre
```

### 7.2 下载并配置GitHub Runner

```bash
# 创建Runner目录
mkdir -p ~/actions-runner && cd ~/actions-runner

# 下载Runner（替换为你的Runner版本）
curl -o actions-runner-linux-x64-2.317.0.tar.gz -L https://github.com/actions/runner/releases/download/v2.317.0/actions-runner-linux-x64-2.317.0.tar.gz

# 解压Runner
tar xzf ./actions-runner-linux-x64-2.317.0.tar.gz

# 配置Runner（按照提示输入GitHub仓库URL和Runner token）
./config.sh

# 安装并启动Runner服务
sudo ./svc.sh install
sudo ./svc.sh start

# 验证Runner状态
sudo ./svc.sh status
```

### 7.3 配置Runner使用Podman

```bash
# 在Runner配置中添加Podman支持
echo "ACTIONS_RUNNER_PODMAN_PATH=$(which podman)" >> ~/actions-runner/.env
echo "DOCKER_HOST=unix:///run/user/$(id -u)/podman/podman.sock" >> ~/actions-runner/.env

# 重启Runner服务
sudo ./svc.sh restart
```

### 7.4 创建GitHub Actions工作流测试

在你的GitHub仓库中创建`.github/workflows/test-podman.yml`文件：

```yaml
name: Test Podman in Runner

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  test-podman:
    runs-on: self-hosted
    steps:
    - uses: actions/checkout@v4
    
    - name: Test Podman version
      run: podman --version
    
    - name: Test Podman run
      run: podman run --rm docker.io/library/alpine:latest echo "Hello from Podman!"
    
    - name: Test Rust build
      run: |
        cargo build
        cargo test
```

## 8. 自动化脚本

### 8.1 环境初始化脚本

创建`init-wsl2-env.sh`脚本，用于自动化初始化WSL2环境：

```bash
#!/bin/bash

# 更新系统
echo "更新系统..."
sudo apt update && sudo apt upgrade -y

# 安装基本依赖
echo "安装基本依赖..."
sudo apt install -y build-essential git curl wget unzip jq

# 安装Podman
echo "安装Podman..."
source /etc/os-release
echo "deb https://download.opensuse.org/repositories/devel:/kubic:/libcontainers:/unstable/xUbuntu_${VERSION_ID}/ /" | sudo tee /etc/apt/sources.list.d/devel:kubic:libcontainers:unstable.list
curl -fsSL https://download.opensuse.org/repositories/devel:/kubic:/libcontainers:/unstable/xUbuntu_${VERSION_ID}/Release.key | sudo gpg --dearmor -o /etc/apt/trusted.gpg.d/libcontainers.gpg
sudo apt update && sudo apt install -y podman

# 配置Podman
echo "配置Podman..."
systemctl --user enable --now podman.socket
echo "alias docker='podman'" >> ~/.bashrc
echo "alias docker-compose='podman-compose'" >> ~/.bashrc

# 安装Podman Compose
echo "安装Podman Compose..."
sudo apt install -y python3-pip
pip3 install podman-compose

# 安装Git
echo "安装Git..."
sudo apt install -y git

# 安装Rust
echo "安装Rust和Cargo..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

echo "WSL2环境初始化完成！请重新登录或运行 'source ~/.bashrc && source ~/.cargo/env' 配置环境变量。"
```

### 8.2 使用脚本初始化环境

```bash
# 赋予脚本执行权限
chmod +x init-wsl2-env.sh

# 运行脚本初始化环境
./init-wsl2-env.sh

# 配置环境变量
source ~/.bashrc
source ~/.cargo/env
```

## 9. 性能优化

### 9.1 配置WSL2资源

在Windows中创建或编辑`%UserProfile%\.wslconfig`文件：

```ini
[wsl2]
memory=8GB
processors=4
swap=4GB
localhostForwarding=true
```

### 9.2 优化Podman性能

```bash
# 编辑Podman配置文件
mkdir -p ~/.config/containers
cat > ~/.config/containers/containers.conf << EOF
[containers]
log_driver = "journald"

default_sysctls = [
  "net.ipv4.ping_group_range=0 2147483647",
]

[engine]
cgroup_manager = "systemd"
events_logger = "journald"
EOF
```

## 10. 安全性配置

### 10.1 配置SELinux（可选）

```bash
# 安装SELinux工具
sudo apt install -y selinux-utils selinux-basics

# 启用SELinux
sudo selinux-activate

# 配置Podman使用SELinux
cat >> ~/.config/containers/containers.conf << EOF

[engine]
selinux = true
EOF
```

### 10.2 配置防火墙

```bash
# 安装UFW防火墙
sudo apt install -y ufw

# 启用防火墙
sudo ufw enable

# 允许SSH连接
sudo ufw allow 22/tcp

# 允许容器registry访问
sudo ufw allow 5000/tcp

# 查看防火墙状态
sudo ufw status
```

## 11. 监控和管理

### 11.1 安装监控工具

```bash
# 安装Podman监控工具
sudo apt install -y podman-docker cockpit-podman

# 安装Prometheus和Grafana（可选）
# sudo apt install -y prometheus grafana
```

### 11.2 配置日志管理

```bash
# 安装日志管理工具
sudo apt install -y systemd-journal-remote logrotate

# 配置Podman日志使用journald
cat >> ~/.config/containers/containers.conf << EOF

[containers]
log_driver = "journald"
EOF
```

## 12. 常见问题和解决方案

### 12.1 Podman无法连接到socket

```bash
# 确保podman.socket服务正在运行
systemctl --user status podman.socket

# 启动服务
systemctl --user enable --now podman.socket

# 检查socket文件
ls -la /run/user/$(id -u)/podman/podman.sock
```

### 12.2 镜像拉取速度慢

```bash
# 配置镜像加速器
cat > ~/.config/containers/registries.conf << EOF
[[registry]]
location = "docker.io"
mirror = [
  "https://registry.docker-cn.com",
  "https://mirror.baidubce.com"
]
EOF
```

### 12.3 权限问题

```bash
# 确保当前用户有足够的权限
sudo chown -R $USER:$USER ~/.config/containers
sudo chmod -R 755 ~/.config/containers
```

## 13. 总结

本指南详细介绍了如何在WSL2环境中部署Podman、Docker、Docker Compose等容器工具，以及Git、Rust、Cargo等开发工具，用于项目代码编译、测试、Podman容器构建与调试，最终用于GitHub私有化Runner。

**主要内容包括：**

- WSL2环境准备和配置
- 容器工具（Podman、Docker、Docker Compose）的安装和配置
- 开发工具（Git、Rust、Cargo）的安装和配置
- Podman仓库配置（本地和远程）
- 代码编译和测试环境配置
- Podman容器构建与调试
- GitHub私有化Runner配置
- 自动化脚本编写和使用
- 性能优化和安全性配置
- 监控和管理
- 常见问题和解决方案

通过本指南，您可以在WSL2环境中成功部署一个完整的容器打包环境，用于开发、测试和CI/CD工作流。