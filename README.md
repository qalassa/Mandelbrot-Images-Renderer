# Mandelbrot-Images-Renderer
This code execute(or actually render) Mandelbrot Set fractals resulting in images as output.
![mandelbrot_0018](https://user-images.githubusercontent.com/109701506/236647025-86de7365-aa89-42ae-ac28-3f6954be8d49.png)
The more the compilation lasts, the more acuration and beauty released.

Compilation:
```
cargo build
```
Running:
```
cargo run
```

You can create an animated video from the images by doing:
```
ffmpeg -framerate 30 -i mandelbrot_%04d.png -c:v libx264 -pix_fmt yuv420p mandelbrot_zoom.mp4
```
where 30 is the number of frames per second(fps).

for example, if you want a 10 second lasting video, you'll need:

10(seconds) * 30(fps) = 300 images.

TODO:

**Animatied version(Redndering slowly until it shapes fully)**

**More Fractals**

**Mathematical based animation**


