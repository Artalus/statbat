#!/bin/bash

B=/sys/class/power_supply/BAT0
#CPU=$(top -bn1 | grep "Cpu(s)" | \
#           sed "s/.*, *\([0-9.]*\)%* id.*/\1/" | \
#           awk '{print 100 - $1}')
CPU=$(cat /proc/loadavg | cut -f1 -d' ')
CAP=$(cat $B/capacity)
#ENRG=$(cat $B/energy_now)
#PWR=$(cat $B/power_now)
STS=$(cat $B/status)
DT=$(date "+%Y.%m.%d %H:%M:%S")
#echo "$DT,$CAP,$ENRG,$PWR,$STS"
echo "$DT,$CAP,$CPU,$STS"
