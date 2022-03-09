# Ray tracing in Rust

I'm currently learning Rust, so I thought... why not implement a ray tracer,
right? So here we are!

This is, as the title already explains, a ray tracer written in Rust in my spare
time. This work is based on [_Ray Tracing in One
Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

## The end result (for now...)

![Random Scene](output/random.ppm)

## Example usage

There are currently two different scenes that you can execute, the `easy` and
the `random`, the later shown in the previous section. An usage example:
```
$ cargo run --release random name_of_the_file
```
The program will then write the ppm format image to the requested file and to
open the file `output/name_of_the_file` using [feh](https://wiki.archlinux.org/title/Feh).

## TODO

* User specified scene and parameters in JSON format
* Lightings
* Textures
* Different shapes
* More to come... (maybe, though)
