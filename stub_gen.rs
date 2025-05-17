use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    // `stub_info` is a function defined by `define_stub_info_gatherer!` macro.
    let stub = qucirc::python::stub_info()?;
    println!(
        "{}",
        stub.modules.keys().cloned().collect::<Vec<_>>().join(", ")
    );
    stub.generate()?;
    Ok(())
}
