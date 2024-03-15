use extendr_api::prelude::*;
use polars::prelude::*;
use plomo::models::sperry;
use std::fs::File;
use toml;

/// Passes a config to a model, then runs it. 
/// @export
#[extendr]
fn model_runner(
    path_to_config: String,
    path_to_data: String, 
    path_to_output: String
) -> String {
    let m = sperry::SperryModel::try_new_from_paths(path_to_config, path_to_data)
        .expect("bad model, fail");

    let o = m.execute(path_to_output);
    
    return o;
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
