import math
import statistics
import time
from pathlib import Path

import mandelbulb
import numpy as np
import typer
from PIL import Image

app = typer.Typer(help="Mandelbulb fractal renderer")


@app.command()
def render(
    width: int = typer.Option(1000, min=1, help="Width of the output image"),
    height: int = typer.Option(600, min=1, help="Height of the output image"),
    power: float = typer.Option(8.0, min=1.0, help="Power of the Mandelbulb"),
    fractal_iterations: int = typer.Option(
        100, min=1, help="Number of iterations for fractal calculation"
    ),
    ray_steps: int = typer.Option(
        256, min=1, help="Number of ray marching steps for rendering"
    ),
    color_mode: str = typer.Option(
        "rgb", help="Color mode for rendering (e.g., 'rgb', 'grayscale')"
    ),
    gif: bool = typer.Option(
        False, help="Whether to render an animated GIF (60 frames)"
    ),
    parallel: bool = typer.Option(True, help="Whether to render frames in parallel"),
    output: str = typer.Option("mandelbulb", help="Output image file name"),
):
    if gif:
        output_path = with_suffix(output, ".gif")
        frames = []
        for frame in range(60):
            cam_x = 2.5 * math.sin(frame * math.pi / 30)
            cam_y = 0.0
            cam_z = 2.5 * math.cos(frame * math.pi / 30)
            print(f"Rendering frame {frame + 1}/60...")
            buffer = mandelbulb.render_mandelbulb(
                width,
                height,
                power,
                fractal_iterations,
                ray_steps,
                cam_x,
                cam_y,
                cam_z,
                color_mode,
                parallel,
            )
            image = buffer_to_image(buffer, width, height)
            frames.append(image)
        frames[0].save(
            output_path,
            save_all=True,
            append_images=frames[1:],
            duration=80,
            loop=0,
        )
        print(f"Rendering complete! Saved as {output_path}")
    else:
        output_path = with_suffix(output, ".png")
        cam_x = 0.0
        cam_y = 0.0
        cam_z = 2.5
        print("Rendering Mandelbulb...")
        buffer = mandelbulb.render_mandelbulb(
            width,
            height,
            power,
            fractal_iterations,
            ray_steps,
            cam_x,
            cam_y,
            cam_z,
            color_mode,
            parallel,
        )
        image = buffer_to_image(buffer, width, height)
        image.save(output_path)
        print(f"Rendering complete! Saved as {output_path}")


@app.command()
def benchmark(
    width: int = typer.Option(1000, min=1, help="Width of the rendered image"),
    height: int = typer.Option(600, min=1, help="Height of the rendered image"),
    power: float = typer.Option(8.0, min=1.0, help="Power of the Mandelbulb"),
    fractal_iterations: int = typer.Option(
        100, min=1, help="Number of iterations for fractal calculation"
    ),
    ray_steps: int = typer.Option(
        256, min=1, help="Number of ray marching steps for rendering"
    ),
    color_mode: str = typer.Option(
        "rgb", help="Color mode for rendering (e.g., 'rgb', 'grayscale')"
    ),
    repeats: int = typer.Option(3, min=1, help="Number of benchmark repetitions"),
    warmup: int = typer.Option(1, min=0, help="Number of warmup runs before measuring"),
):
    cam_x = 0.0
    cam_y = 0.0
    cam_z = 2.5

    def run_once(parallel: bool) -> float:
        start = time.perf_counter()
        mandelbulb.render_mandelbulb(
            width,
            height,
            power,
            fractal_iterations,
            ray_steps,
            cam_x,
            cam_y,
            cam_z,
            color_mode,
            parallel,
        )
        end = time.perf_counter()
        return end - start

    print("Warming up...")
    for _ in range(warmup):
        run_once(parallel=False)
        run_once(parallel=True)

    print(f"Running benchmark: {width}x{height}, repeats={repeats}")

    sequential_times = []
    parallel_times = []

    for i in range(repeats):
        print(f"Sequential run {i + 1}/{repeats}...")
        sequential_times.append(run_once(parallel=False))

    for i in range(repeats):
        print(f"Parallel run {i + 1}/{repeats}...")
        parallel_times.append(run_once(parallel=True))

    sequential_avg = statistics.mean(sequential_times)
    parallel_avg = statistics.mean(parallel_times)
    speedup = sequential_avg / parallel_avg if parallel_avg > 0 else float("inf")

    print()
    print("Benchmark results:")
    print(f"Sequential avg: {sequential_avg:.4f}s")
    print(f"Parallel avg:   {parallel_avg:.4f}s")
    print(f"Speedup:        {speedup:.2f}x")

    print()
    print("Detailed results:")
    print(f"Sequential runs: {[round(t, 4) for t in sequential_times]}")
    print(f"Parallel runs:   {[round(t, 4) for t in parallel_times]}")


def buffer_to_image(buffer: bytes, width: int, height: int) -> Image.Image:
    flat_array = np.frombuffer(buffer, dtype=np.uint8)
    image_array = flat_array.reshape((height, width, 3))
    return Image.fromarray(image_array)


def with_suffix(output: str, suffix: str) -> Path:
    path = Path(output)
    return path if path.suffix else path.with_suffix(suffix)


if __name__ == "__main__":
    app()
