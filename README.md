# Arc Macro

A simple crate providing one convenience macro for creating an `Arc[T]`. This crate is based off [this](https://www.youtube.com/watch?v=A4cKi7PTJSs&t=34s) video by [Logan Smith](https://www.youtube.com/@_noisecode) and a subsequent comment on it from [@Choroalp](https://www.youtube.com/@Choroalp). The entire concept of this is that `Arc[T]`is a very useful tool, however the convenience of simply using `vec![]` is too powerful. Thus, the `arc![]` macro is born! For the pros of using `Arc` over `Vec`, please check out the aforementioned video.

# Usage

```rust
let my_arc = arc![1, 2, 3];
```

That's it, no hassle, no nothing. It's just a wrapper for `Arc::new()`.
