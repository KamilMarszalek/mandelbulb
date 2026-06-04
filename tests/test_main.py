import importlib.util
import sys
import types
from pathlib import Path

from typer.testing import CliRunner


def import_main_with_fake_renderer(monkeypatch):
    def fake_render_mandelbulb(width, height, *args):
        return bytes([0, 0, 0] * width * height)

    fake_mandelbulb = types.SimpleNamespace(render_mandelbulb=fake_render_mandelbulb)
    monkeypatch.setitem(sys.modules, "mandelbulb", fake_mandelbulb)
    sys.modules.pop("main", None)

    module_path = Path(__file__).resolve().parents[1] / "main.py"
    spec = importlib.util.spec_from_file_location("main", module_path)
    module = importlib.util.module_from_spec(spec)
    sys.modules["main"] = module
    spec.loader.exec_module(module)
    return module


def test_with_suffix_adds_png_for_basename(monkeypatch):
    main = import_main_with_fake_renderer(monkeypatch)

    assert str(main.with_suffix("mandelbulb", ".png")).endswith(".png")


def test_with_suffix_does_not_duplicate_existing_png_suffix(monkeypatch):
    main = import_main_with_fake_renderer(monkeypatch)

    assert main.with_suffix("mandelbulb.png", ".png").name == "mandelbulb.png"


def test_buffer_to_image_returns_rgb_image_with_expected_size(monkeypatch):
    main = import_main_with_fake_renderer(monkeypatch)
    buffer = bytes(
        [
            255,
            0,
            0,
            0,
            255,
            0,
            0,
            0,
            255,
            255,
            255,
            255,
        ]
    )

    image = main.buffer_to_image(buffer, width=2, height=2)

    assert image.size == (2, 2)
    assert image.mode == "RGB"


def test_render_command_uses_fake_renderer_and_writes_png(monkeypatch, tmp_path):
    main = import_main_with_fake_renderer(monkeypatch)
    output = tmp_path / "rendered"
    runner = CliRunner()

    result = runner.invoke(
        main.app,
        [
            "render",
            "--width",
            "2",
            "--height",
            "2",
            "--fractal-iterations",
            "1",
            "--ray-steps",
            "1",
            "--output",
            str(output),
        ],
    )

    assert result.exit_code == 0
    assert output.with_suffix(".png").exists()
