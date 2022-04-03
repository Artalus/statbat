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
set format x "%H:%M\n%b.%d"

# tics are set by frequency
# for x unit is second
set mxtics 3
set mytics 5
set xtics 3600


set style line 100 lt 1 lc rgb "gray" lw 2
set style line 101 lt 0.5 lc rgb "gray" lw 1
set grid mytics ytics ls 100, ls 101
set grid mxtics xtics ls 100, ls 101

set term png size 1920,1000
set output 'outstatbat.png'
set datafile sep ','
plot 'statbat.csv' using 1:2 lt rgb 'red' w l title 'capacity'
