fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindgen::builder()
        .header("scd4x_i2c.h")        
        .header("sensirion_common.h")        
        .header("sensirion_config.h")        
        .header("sensirion_i2c.h")        
        .header("sensirion_i2c_hal.h")
        .generate()
        .write_to_file(out_dir.join("bindings.rs"));
}
