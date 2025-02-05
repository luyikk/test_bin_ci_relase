
#[inline(always)]
fn version() -> &'static str {
    concat! {
    "\n",
    "==================================version info==================================",
    "\n",
    "Build Timestamp:", env!("VERGEN_BUILD_TIMESTAMP"), "\n",
    "Build System:",env!("VERGEN_SYSINFO_OS_VERSION"), "\n",
    "GIT BRANCH:", env!("VERGEN_GIT_BRANCH"), "\n",
    "GIT COMMIT DATE:", env!("VERGEN_GIT_COMMIT_TIMESTAMP"), "\n",
    "GIT SHA:", env!("VERGEN_GIT_SHA"), "\n",
    "Debug:", env!("VERGEN_CARGO_DEBUG"), "\n",
    "Opt level:", env!("VERGEN_CARGO_OPT_LEVEL"), "\n",
    "==================================version end==================================",
    "\n",
    }
}

fn main() {
    let result = add(2, 2);
    println!("2 + 2 = {}", result);

    let result = add(2, 3);
    println!("2 + 3 = {}", result);

    println!("{}", version());
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}