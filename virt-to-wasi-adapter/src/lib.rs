#![no_main]

wit_bindgen::generate!({
    path: "../wit",
    world: "virt-to-wasi-adapter",
    exports: {
       "local:virt/incoming-http": Adapter,
       "wasi:http/incoming-handler": Adapter
    }
});

use wasi::{
    cli::run,
    http::types::{IncomingRequest, ResponseOutparam},
};

struct Adapter;

static mut PARAMS: Option<(IncomingRequest, ResponseOutparam)> = None;

impl exports::local::virt::incoming_http::Guest for Adapter {
    fn get_params() -> (IncomingRequest, ResponseOutparam) {
        unsafe { PARAMS.take() }.unwrap()
    }
}

impl exports::wasi::http::incoming_handler::Guest for Adapter {
    fn handle(request: IncomingRequest, outparam: ResponseOutparam) {
        unsafe {
            PARAMS = Some((request, outparam));
        }
        run::run().unwrap();
    }
}
