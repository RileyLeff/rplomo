use extendr_api::prelude::*;

/// Passes a config to a model, then runs it. 
/// @export
#[extendr]
fn model_runner(model: &str, path_to_config: &str, path_to_output: &str, verbose: bool) -> &str {
    // check if model is valid
        // if verbose, print out whether it's valid

    // check if config is valid for the model
        // if verbose, print out whether it's valid

    // fire up the model
        
    // if verbose, print out whether it worked / when it's done
    // if verbose, print out where to find output

    println!("howdy");
    "ToDo"
}

/// Passes a config to a model, then runs it. 
/// @export
#[extendr]
fn config_generator(model: &str, save_to_path: &str, verbose: bool) -> &str {
    println!("howdy");
    "ToDo"
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rplomo;
    fn model_runner;
}
