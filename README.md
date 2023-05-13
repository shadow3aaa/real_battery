## 此程序模块用于对于某些安卓设备的电量和真实电量略有偏差的纠正
___
## 编译
### 安装rust
```
# wsl/linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# termux(not in proot)
apt install rust
# or
pkg add rust
```
### 下载源代码
* #### 使用git
```
git clone https://github.com/shadow3aaa/real_battery.git --depth=1
```
* #### 或者，从该项目[release](https://github.com/shadow3aaa/real_battery/releases)下载最新source_code.zip
### 编译二进制程序
```
# 进入项目文件夹
cd real_battery
# 编译
./build.sh
```
#### 打包
```
./pack_module.sh
```
* 然后，项目根目录会出现Real_Battery.zip
* 用magisk刷入即可安装
___
## 安装
* 下载最新[release](https://github.com/shadow3aaa/real_battery/releases)
* 在magisk内安装