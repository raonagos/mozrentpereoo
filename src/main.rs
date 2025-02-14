#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use mozrentpereoo::shared::run_server;

    run_server().await;
}

#[cfg(not(feature = "server"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
