**Brute is sample program for funny experiments in rust language**

```
Brute 0.1.0

USAGE:
    brute [OPTIONS] --hash <hash> --max <plaintext-max-length> --min <plaintext-min-length>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --algo <algorithm>              Algorithms md5 [default: md5]
        --alpha <alphabet>              Alphabet numeric (0123456789) [default: numeric]
        --hash <hash>                   Hash md5
        --max <plaintext-max-length>    Max plaintext length
        --min <plaintext-min-length>    Min plaintext length
        --threads <threads>             Threads [default: 10]
```

```
./brute --hash 2e9ec317e197819358fbc43afca7d837 --min 8 --max 8
Search hash 2e9ec317e197819358fbc43afca7d837 MD5 NUMERIC Plaintext length 8-8 Threads 10.
Subspace 100000000/(per thread 10000000) for current plaintext length 8

Found "2e9ec317e197819358fbc43afca7d837" 01234567
End time 77.016653124s
```
