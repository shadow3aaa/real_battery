MODDIR=${0%/*}
pkill -9 real_batt
chmod a+x "$MODDIR/real_batt"
{
    while [ ! -f /sys/class/power_supply/battery/capacity ]; do
        sleep 1
    done
    nohup $MODDIR/real_batt >/dev/null 2>&1
} &