# The data (each line is a format!("{} {}\n", time_s, temp_k)) needs to be
# concatenated to the end of this file then piped into Gnuplot.
# It will write PNG data to stdout.

set terminal png

# Format X-Axis
set xdata time
set timefmt "%s"
set format x "%Y/%m/%d %H:%M:%S"
set xtics rotate

# Format Y-Axis
set ylabel "Temp (Celsius)"

# Don't print a legend, there's only one line
set key off

# Do the plotting; data should be concatenated to this file.
plot '-' using 1:2 with linespoints
