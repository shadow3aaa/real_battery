MODDIR=${0%/*}
{
    pkill -9 real_batt >/dev/null 2>&1
    chmod a+x "$MODDIR/real_batt"
    nohup "$MODDIR/real_batt" >/dev/null 2>&1
} &