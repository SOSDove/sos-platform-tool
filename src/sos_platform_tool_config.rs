use std::env;

pub fn use_optimized_image() -> bool {
    let use_optimized_image = env::var("USE_OPTIMIZED_IMAGE").unwrap_or("false".parse().unwrap());
    let use_optimized_image: bool = use_optimized_image.parse().unwrap();
    use_optimized_image
}