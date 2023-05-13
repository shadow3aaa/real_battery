function pred() {
	local error_message=$1
	echo -e "\033[31m$error_message\033[0m" >&2
}

function pgreen() {
	local error_message=$1
	echo -e "\033[32m$error_message\033[0m" >&2
}

if [[ ! -d ${0%/*}/Real_Battery || ! -f ${0%/*}/Real_Battery/real_batt ]]; then
	pred "打包失败，也许还没有编译?"
	exit -1
fi

cd ${0%/*}/Real_Battery
zip -r -X9 -FS ${0%/*}/../Real_Battery.zip ./

if [ $? -eq 0 ]; then
	pgreen "打包magisk模块成功"
else
	pred "打包为magisk模块失败"
fi
