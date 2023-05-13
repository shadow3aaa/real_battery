ui_print "让部分支持的设备显示更符合实际状态的电量百分比"
chmod a+x "$MODPATH/real_batt"
killall -9 real_batt >/dev/null 2>&1
nohup "$MODPATH/real_batt" >/dev/null 2>&1 &
if [ "$(pidof real_batt)" = "" ]; then
    ui_print "运行失败"
    abort
fi
ui_print "已运行"
ui_print "由于采用方法不同的原因"
ui_print "没有bms的机型可能需要安装完重启生效"