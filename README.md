# DLS App
Simple app to get the revised target score for the chasing team in a cricket match.
It uses the [Duckworth-Lewis-Stern method](https://en.wikipedia.org/wiki/Duckworth%E2%80%93Lewis%E2%80%93Stern_method)
to calculate the revised target score.

## Tools used
This app is build in Rust using the [leptos framework](https://github.com/leptos-rs/leptos).

# Running locally
You'll need the `Rust` toolchain. `trunk` to build and serve the app. and the `wasm32-unknown-unknown` target.
## Why though?
Made this just to get a basic feel for what it's like making frontend components using
leptos and Rust

## Methodology
the methodology for the calculations follows this [ICC publication](https://images.icc-cricket.com/image/upload/prd/orlbya4cqyhqaceje3b2.pdf)

### Thanks
to Yuvraj Khetan for helping me understand what DLS is and what a useful interface would look like

