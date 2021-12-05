# yes

yes command clone for speed test

Each commit message includes the result of the speed test with the following command.
The implement consulted the websites described in the reference.

## Benchmark

```sh
cargo run --release | pv -r > /dev/null
```

## Reference

- https://postd.cc/a-little-story-about-the-yes-unix-command/ (Japnese)
- https://keens.github.io/blog/2017/10/05/rustdekousokunahyoujunshutsuryoku/ (Japanese)
