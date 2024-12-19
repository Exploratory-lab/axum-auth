//! TODO: Add a description

// std library imports
use std::process;

// Local imports
use axum_auth::run_app;

#[tokio::main]
async fn main() {
    match run_app().await {
        Ok(_) => println!("Application stopped with no error reported."),
        Err(e) => {
            eprintln!("Application reported error: {}", e);
            process::exit(1);
        }
    };
}
