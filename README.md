Run with `cargo bench`.

Result on i9-9900K (3.60 GHz) + 64GB RAM

```
zune encode qoi         time:   [5.9150 ms 6.0006 ms 6.0984 ms]
image-rs encode qoi     time:   [3.9894 ms 4.0335 ms 4.0826 ms]
zune encode jpeg        time:   [3.3753 ms 3.3994 ms 3.4257 ms]
zune encode png         time:   [2.7774 ms 2.8188 ms 2.8650 ms]

zune decode qoi         time:   [2.0401 ms 2.0505 ms 2.0617 ms]
image-rs decode qoi     time:   [1.5920 ms 1.5990 ms 1.6066 ms]
zune decode jpeg        time:   [942.27 µs 948.51 µs 955.29 µs]
zune decode png         time:   [853.24 µs 869.38 µs 886.10 µs]
```
