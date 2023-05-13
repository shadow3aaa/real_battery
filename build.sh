#!/usr/bin/bash
TAGS="--release -Z unstable-options --out-dir=${0%/*}/target/build"
function pred() {
	local error_message=$1
	echo -e "\033[31m$error_message\033[0m" >&2
}

function pgreen() {
	local error_message=$1
	echo -e "\033[32m$error_message\033[0m" >&2
}

# try taregt aarch64-linux-android
rustup target add aarch64-linux-android 2>&1 >/dev/null
cargo build $TAGS --target=aarch64-linux-android
if [[ ! $? -eq 0 ]]; then
	pred "编译到target: aarch64-linux-android失败!"
	pred "检查是否缺少相关库"
	pgreen "尝试编译到target: aarch64-unknown-linux-musl，在安卓上运行效率较aarch64-linux-android低，但是内存占用小"
	# try target aarch64-unknown-linux-musl
	rustup target add aarch64-unknown-linux-musl 2>&1 >/dev/null
	cargo build $TAGS --target=aarch64-unknown-linux-musl
	if [[ ! $? -eq 0 ]]; then
		pred "编译到target: aarch64-unknown-linux-musl 失败!"
		pgreen "尝试编译到target: aarch64-unknown-linux-gnu，该target二进制文件大小较大"
		# try target aarch64-unknown-linux-gnu
		export RUSTFLAGS="-C target-feature=+crt-static"
		rustup target add aarch64-unknown-linux-gnu 2>&1 >/dev/null
		cargo build $TAGS aarch64-unknown-linux-gnu
	fi
fi
[[ ! $? -eq 0 ]] &&
	\pred "编译失败" &&
	\exit -1

# 优化体积
sstrip ${0%/*}/target/build/real_battery

pgreen "编译成功，复制到模块"
cp -f ${0%/*}/target/build/real_battery ${0%/*}/Real_Battery/real_batt
