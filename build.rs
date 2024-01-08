
fn main() {
    // println!("cargo:rustc-env=LIB_PADDLE_C_INSTALL_DIR=paddle_inference_c_install_dir");
    // println!("cargo:rustc-env=DYLD_FALLBACK_LIBRARY_PATH=paddle_inference_c_install_dir/paddle/lib");
    println!("cargo:rustc-link-search=native=paddle_inference_c_install_dir/paddle/lib");
}
