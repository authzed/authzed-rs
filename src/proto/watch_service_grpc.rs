// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_WATCH_SERVICE_WATCH: ::grpcio::Method<super::watch_service::WatchRequest, super::watch_service::WatchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/WatchService/Watch",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct WatchServiceClient {
    client: ::grpcio::Client,
}

impl WatchServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        WatchServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn watch_opt(&self, req: &super::watch_service::WatchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::watch_service::WatchResponse>> {
        self.client.server_streaming(&METHOD_WATCH_SERVICE_WATCH, req, opt)
    }

    pub fn watch(&self, req: &super::watch_service::WatchRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::watch_service::WatchResponse>> {
        self.watch_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait WatchService {
    fn watch(&mut self, ctx: ::grpcio::RpcContext, req: super::watch_service::WatchRequest, sink: ::grpcio::ServerStreamingSink<super::watch_service::WatchResponse>);
}

pub fn create_watch_service<S: WatchService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_WATCH_SERVICE_WATCH, move |ctx, req, resp| {
        instance.watch(ctx, req, resp)
    });
    builder.build()
}
