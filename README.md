# BigHeroX Robocup Rust版本代码

## 介绍
BigHeroX Robocup Rust版本代码

---

## 当前进度/目标

- [ ] 界面：按钮、图像
- - [ ] 状态内容集中在左方和上方的新布局
- - [ ] 球场内选中球员或球
- - [ ] 识别鼠标操作指令
- [ ] 输入：机器人下位机
- [ ] 输入：MPU
- - [ ] 测试windows crate能否胜任
- [ ] 输入：全景相机
- - [ ] 尝试使用bindgen方式实现自动化链接此库，做到类似于OpenCV crate的效果
- [ ] 输入：网络通信
- - [ ] 与原教练机通信（卡住了，只能与老教练机进行一轮数据传输，原因未知）

---

## 项目文件获取

需要安装[Python](https://python.org)和[Git](https://git-scm.com)以执行以下命令。

1. 克隆此仓库

> 建议在一个空文件夹中执行此操作，因为后续操作会在当前目录下生成好几个文件夹。
```bash
git clone http://115.157.210.249:8080/bigherox-robocup/bigherox-robocup.git
```

2. 运行此仓库下的更新脚本

```bash
python ./bigherox-robocup/scripts/update_remote.py
```

## 本地同步项目

### 准备
1. 克隆此仓库。
2. 运行一次`update_local.py`，生成`update_local_path`文件后，Ctrl+C退出。 
3. 修改生成的`update_local_path`文件，指向总项目目录。（不是子文件夹的各个仓库目录！）

### 同步
```bash
python ./bigherox-robocup/scripts/update_local.py
```

---

## 编译运行

### 准备：编译环境配置
按照[`编译环境配置.md`](./docs/编译环境配置.md)中的方式，配置好所有编译环境。

可以运行以下命令，检查环境是否已经配置完成：

```bash
python ./bigherox-robocup/scripts/check_compile_environment.py
```

### 教练机
```bash
cd ../bigherox-robocup/
cargo run --release --bin bigherox-robocup-coach
```

### 球员机：进攻球员
```bash
cd ../bigherox-robocup/
cargo run --release --bin bigherox-robocup-striker
```

目前只能使用Release模式编译外部C++库部分，原因如下：
- OpenCV官方版提供的lib和dll文件中，opencv_world480d库无法正常被`opencv`crate链接，仅可使用opencv_world480库。
- 我们使用MSVC，在Debug模式下手动编译的库，与opencv_world480库的常量有冲突。

首次编译大概需要5到15分钟。

调试程序时，需要将Rust部分和每个C++模块分开调试。

---

## 程序信息

### 全局坐标系
单位长度：米。

右手坐标系：场地中心为零点，敌方球门方向为x轴正方向，正前方朝着敌方球门时，左侧为y轴正方向。

### 文件目录结构
文档待完善。
