#!/bin/sh

rm -f target/out.dat
if [ $# -eq 1 ]; then
    if [ $1 -lt 2 ]; then
        echo "error: need at least 2 runs to compute statistics"
        exit 1
    else
        runs=$1
    fi
else
    runs=5
fi
sizes=(256 512 768 1024 1280 1536 1792)
target=target/release/matrix_multiplication

if [ ! -f "$target" ]; then
    cargo b --release
fi

printf "     \033[1;32mRunning\033[0m release [optimized] $runs runs per matrix size\n\n"
before=$(date +%s.%N)
for size in "${sizes[@]}"; do
    printf "Matrix %zux%zu...\t" "$size" "$size"
    taskset -c 7 $target $size $runs | grep -e "[0-9]" >> target/out.dat
    echo "done"
done
after=$(date +%s.%N)

elapsed=$(echo "scale=3; $after - $before" | bc -l)
printf "\nBenchmarks finished in %.2f seconds\n" "$elapsed"

gnuplot bench.gp
printf "Data: \'target/out.dat\'\n"
printf "Plot: \'target/bench.png\'\n"

exit 0
