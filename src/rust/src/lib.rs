use extendr_api::prelude::*;
use plomo::models::sperry::{SperryModel, SperryData, SperryConfig};

/// Passes a config to a model, then runs it. 
/// @export
#[extendr]
fn model_runner(path_to_config: String, path_to_output: String, verbose: bool) -> Result<f64> {
    let to_out: std::path::PathBuf = path_to_config.into();
    let the_model = SperryModel::new();
}

// Passes a config to a model, then runs it. 
// @export
// #[extendr]
// fn config_generator(model: &str, save_to_path: &str, verbose: bool) -> &str {
//     println!("howdy");
//     "ToDo"
// }

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rplomo;
    fn model_runner;
}
