ui_print "让部分支持的设备显示更符合实际状态的电量百分比"
pkill -9 real_batt
chmod a+x "$MODPATH/real_batt"
nohup "$MODPATH/real_batt" >/dev/null 2>&1 &
if [[ ! -d /sys/class/power_supply/bms ]]; then
	ui_print "由于采用方法不同的原因"
	ui_print "你的机型需要安装完重启才可能生效"
else
	ui_print "你的机型似乎能马上生效"
	if [ "$(pidof real_batt)" = "" ]; then
		ui_print "运行失败"
		abort
	fi
fi
