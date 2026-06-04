mod color;
mod renderer;
mod sdf;
mod vec3;

use pyo3::{
    Bound, PyResult, pymodule,
    types::{PyModule, PyModuleMethods},
    wrap_pyfunction,
};

use renderer::render_mandelbulb;

#[pymodule]
fn mandelbulb(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(render_mandelbulb, m)?)?;
    Ok(())
}
