use anyhow::Result;
use vergen::EmitBuilder;

fn main() -> Result<()> {
    // Generate the default 'cargo:' instruction output
    EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .all_git()
        .all_rustc()
        .all_sysinfo()
        .emit()
}
