DST Visualization Tool
======================

This is a proof-of-concept tool for previewing machine embroidery designs.
It offloads the bulk of the drawing to the GPU for efficiency with only minimal CPU preprocessing.
It is written in Rust and uses the wgpu API (which supports Vulkan, Metal, and DX12).

To compile and run the program, run
```
cargo run -- ./path/to/design.dst
```
