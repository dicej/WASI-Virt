#![no_main]

wit_bindgen::generate!({
    path: "../wit",
    world: "wasi-to-virt-adapter",
    exports: {
       "wasi:cli/run": Adapter,
    }
});

use {local::virt::incoming_http, wasi::http::incoming_handler};

struct Adapter;

impl exports::wasi::cli::run::Guest for Adapter {
    fn run() -> Result<(), ()> {
        let (request, outparam) = incoming_http::get_params();
        incoming_handler::handle(request, outparam);
        Ok(())
    }
}
