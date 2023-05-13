MODDIR=${0%/*}
pkill -9 real_batt
chmod a+x "$MODDIR/real_batt"
if [[ ! -d /sys/class/power_supply/bms ]]; then
    {
        while [ ! -f /sys/class/power_supply/battery/capacity ]; do
            sleep 1
        done
    	$MODDIR/real_batt --init_mount
    	nohup $MODDIR/real_batt >/dev/null 2>&1 &
	} &
else
    nohup $MODDIR/real_batt >/dev/null 2>&1 &
fi
