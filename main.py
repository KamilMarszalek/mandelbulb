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
    output: str = typer.Option("mandelbulb.png", help="Output image file name"),
):
    print(f"Rendering image {width}x{height}...")
    buffer = mandelbulb.render_mandelbulb(width, height, power, max_steps)
    flat_array = np.frombuffer(buffer, dtype=np.uint8)
    image_array = flat_array.reshape((height, width, 3)).astype(np.uint8)
    image = Image.fromarray(image_array)
    image.save(output)
    print(f"Image saved to {output}")


if __name__ == "__main__":
    app()
