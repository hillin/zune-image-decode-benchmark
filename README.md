Run with `cargo bench`.

Result on i9-9900K (3.60 GHz) + 32GB RAM

```
encode qoi              time:   [5.8880 ms 5.9623 ms 6.0478 ms]
encode jpeg             time:   [3.3849 ms 3.4074 ms 3.4323 ms]
encode png              time:   [2.7700 ms 2.8185 ms 2.8748 ms]

decode qoi              time:   [2.0618 ms 2.0755 ms 2.0910 ms]
decode jpeg             time:   [937.84 µs 946.73 µs 956.73 µs]
decode png              time:   [828.25 µs 839.40 µs 850.99 µs]
```
