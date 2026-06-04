import math

import mandelbulb
import numpy as np
import typer
from PIL import Image

app = typer.Typer(help="Mandelbulb fractal renderer")


@app.command()
def render(
    width: int = typer.Option(1920, help="Width of the output image"),
    height: int = typer.Option(1080, help="Height of the output image"),
    power: int = typer.Option(8, help="Power of the Mandelbulb"),
    max_steps: int = typer.Option(
        100, help="Maximum number of iterations of ray marching"
    ),
    color_mode: str = typer.Option(
        "rgb", help="Color mode for rendering (e.g., 'rgb', 'grayscale')"
    ),
    is_gif: bool = typer.Option(
        False, help="Whether to render an animated GIF (60 frames)"
    ),
    output: str = typer.Option("mandelbulb", help="Output image file name"),
):
    if is_gif:
        output += ".gif"
        frames = []
        for frame in range(60):
            cam_x = 2.5 * math.sin(frame * math.pi / 30)
            cam_y = 0.0
            cam_z = 2.5 * math.cos(frame * math.pi / 30)
            print(f"Rendering frame {frame + 1}/60...")
            buffer = mandelbulb.render_mandelbulb(
                width, height, power, max_steps, cam_x, cam_y, cam_z, color_mode
            )
            flat_array = np.frombuffer(buffer, dtype=np.uint8)
            image_array = flat_array.reshape((height, width, 3)).astype(np.uint8)
            image = Image.fromarray(image_array)
            frames.append(image)
        frames[0].save(
            output,
            save_all=True,
            append_images=frames[1:],
            duration=80,
            loop=0,
        )
        print(f"Rendering complete! Saved as {output}")
    else:
        output += ".png"
        cam_x = 0.0
        cam_y = 0.0
        cam_z = 2.5
        print("Rendering Mandelbulb...")
        buffer = mandelbulb.render_mandelbulb(
            width, height, power, max_steps, cam_x, cam_y, cam_z, color_mode
        )
        flat_array = np.frombuffer(buffer, dtype=np.uint8)
        image_array = flat_array.reshape((height, width, 3)).astype(np.uint8)
        image = Image.fromarray(image_array)
        image.save(output)
        print(f"Rendering complete! Saved as {output}")


if __name__ == "__main__":
    app()
