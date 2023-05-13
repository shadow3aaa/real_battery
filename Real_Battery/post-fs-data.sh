MODDIR=${0%/*}
pkill -9 real_batt >/dev/null 2>&1
chmod a+x "$MODDIR/real_batt"
if [[ ! -d /sys/class/power_supply/bms ]]; then
    # 堵塞启动直到init完成
    $MODDIR/real_batt --init
fi
nohup $MODDIR/real_batt >/dev/null 2>&1 &