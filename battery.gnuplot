#!/usr/bin/env gnuplot
set title 'battery stats'
set ylabel 'percent'
set xlabel 'time'
set key top left autotitle columnheader
set autoscale
set grid

set xdata time
set style data lines
set timefmt '%Y.%m.%d %H:%M:%S'
set format x "%m.%d\n%H:%M"

set term png
set output 'outstatbat.png'
set datafile sep ','
plot 'statbat.csv' using 1:2 lt rgb 'red' w l title 'capacity'
