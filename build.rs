fn main() {
    use std::env;
    use std::path::PathBuf;

    // Get home directory
    let home = dirs::home_dir().expect("No home directory found");

    // Define external SDK paths
    let emlib = home.join(".emlib");
    let cmsis = home.join(".cmsis");
    let mg24 = home.join(".mg24");

    // Validate paths
    assert!(emlib.exists(), "Missing ~/.emlib");
    assert!(cmsis.exists(), "Missing ~/.cmsis");
    assert!(mg24.exists(), "Missing ~/.mg24");

    // Tell Cargo when to rerun
    println!("cargo:rerun-if-changed=build.rs");

    // Compiler config
    let mut build = cc::Build::new();

    build
        .compiler("arm-none-eabi-gcc")
        // Your wrappers
        .file("wrapper/gpio_wrap.c")
        .file("wrapper/cmu_wrap.c")
        .file("wrapper/system_clock_stubs.c")
        // EMLIB sources (from home dir)
        .file(emlib.join("src/em_gpio.c"))
        .file(emlib.join("src/em_cmu.c"))
        .file(emlib.join("src/em_core.c"))
        .file(emlib.join("src/em_emu.c"))
        // Include paths
        .include("wrapper")
        .include(emlib.join("inc"))
        .include(cmsis.join("Core/Include"))
        .include(mg24.join("Include"))
        .include(cmsis.join("common/inc"))
        // Defines
        .define("EFR32MG24B220F1536IM48", None)
        // Flags
        .flag("-mthumb")
        .flag("-mfpu=fpv5-sp-d16")
        .flag("-mfloat-abi=hard")
        .flag("-std=c99")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-fno-exceptions")
        .flag("-fno-unwind-tables")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-sign-compare")
        .flag_if_supported("-Wno-unused-function")
        .compile("emlib_wrap");
}
