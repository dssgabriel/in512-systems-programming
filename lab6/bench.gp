set terminal pngcairo size 1080, 720
set output "target/bench.png"
set title "Naive matrix multiplication"
set xlabel "Matrix size"
set ylabel "Latency in seconds"
set grid
plot "target/out.dat" u 1:2 w l t "Mean time",\
	 "target/out.dat" u 1:2:($2-$3):($2+$3) w yerrorbars t "Standard deviation"
