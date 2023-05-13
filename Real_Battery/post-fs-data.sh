MODDIR=${0%/*}
pkill -9 real_batt >/dev/null 2>&1
chmod a+x "$MODDIR/real_batt"
if [[ ! -d /sys/class/power_supply/bms ]]; then
	# 堵塞启动直到init完成
	# 防止卡开机，10秒后自动放弃
	timeout 10 $MODDIR/real_batt --init_mount
fi
nohup $MODDIR/real_batt >/dev/null 2>&1 &
