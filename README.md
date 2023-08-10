### Generate input vectors:

```
cargo run --release --bin generate -- -f 0 -t 1000000 -o 0+10m.txt
cargo run --release --bin generate -- -f 20000000 -t 120000000 -o 20m+100m.txt
cargo run --release --bin generate -- -f 200000000 -t 1200000000 -o 200m+1000m.txt
```


### Benchmark set creation times:

Results are from an M2 Macbook Pro w/ 64GB RAM and 12 logical cpus


time fst set 0+10m.txt 0+10m.fst
fst set 0+10m.txt 0+10m.fst  131.73s user 2.85s system 276% cpu 48.752 total

time fst set 0+10m.txt 0+10m.fst2
fst set 0+10m.txt 0+10m.fst2  132.16s user 2.85s system 276% cpu 48.859 total

fst set 20m+100m.txt 20m+100m.fst  1884.89s user 37.06s system 336% cpu 9:31.89 total
fst set 20m+100m.txt 20m+100m.fst2  1940.11s user 39.09s system 338% cpu 9:44.64 total
fst set 20m+100m.txt 20m+100m.fst3  2125.21s user 41.04s system 289% cpu 12:29.30 total

fst set 20m+100m.txt 20m+100m.fst5 --batch-size 555556  1553.59s user 31.01s system 331% cpu 7:58.57 total # 12 logical cpus, 15 fd limit => 100 000 000 / 12 / 15 <= 555 556 ==> initial + 2 generations of merges

fst set 20m+100m.txt 20m+100m.fst6 --batch-size 555556  1546.64s user 29.72s system 330% cpu 7:57.32 total # 12 logical cpus, 15 fd limit => 100 000 000 / 12 / 15 <= 555 556 ==> initial + 2 generations of merges

fst set 20m+100m.txt 20m+100m.fst7 --batch-size 37038  2102.10s user 42.66s system 408% cpu 8:44.73 total # 12 logical cpus, 15 fd limit => 100 000 000 / 12 / 15 / 15 <= 37038 ==> initial + 3 generations of merges

fst set 20m+100m.txt 20m+100m.fst8 --batch-size 1000000 --fd-limit 101  960.88s user 17.77s system 224% cpu 7:15.76 total # initial + 1 generations (input file length is one off)

### Benchmark set union times:

