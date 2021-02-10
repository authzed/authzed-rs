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

const METHOD_NAMESPACE_SERVICE_READ_CONFIG: ::grpcio::Method<super::namespace_service::ReadConfigRequest, super::namespace_service::ReadConfigResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/NamespaceService/ReadConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_NAMESPACE_SERVICE_WRITE_CONFIG: ::grpcio::Method<super::namespace_service::WriteConfigRequest, super::namespace_service::WriteConfigResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/NamespaceService/WriteConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct NamespaceServiceClient {
    client: ::grpcio::Client,
}

impl NamespaceServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        NamespaceServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn read_config_opt(&self, req: &super::namespace_service::ReadConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::namespace_service::ReadConfigResponse> {
        self.client.unary_call(&METHOD_NAMESPACE_SERVICE_READ_CONFIG, req, opt)
    }

    pub fn read_config(&self, req: &super::namespace_service::ReadConfigRequest) -> ::grpcio::Result<super::namespace_service::ReadConfigResponse> {
        self.read_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_config_async_opt(&self, req: &super::namespace_service::ReadConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::namespace_service::ReadConfigResponse>> {
        self.client.unary_call_async(&METHOD_NAMESPACE_SERVICE_READ_CONFIG, req, opt)
    }

    pub fn read_config_async(&self, req: &super::namespace_service::ReadConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::namespace_service::ReadConfigResponse>> {
        self.read_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_config_opt(&self, req: &super::namespace_service::WriteConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::namespace_service::WriteConfigResponse> {
        self.client.unary_call(&METHOD_NAMESPACE_SERVICE_WRITE_CONFIG, req, opt)
    }

    pub fn write_config(&self, req: &super::namespace_service::WriteConfigRequest) -> ::grpcio::Result<super::namespace_service::WriteConfigResponse> {
        self.write_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_config_async_opt(&self, req: &super::namespace_service::WriteConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::namespace_service::WriteConfigResponse>> {
        self.client.unary_call_async(&METHOD_NAMESPACE_SERVICE_WRITE_CONFIG, req, opt)
    }

    pub fn write_config_async(&self, req: &super::namespace_service::WriteConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::namespace_service::WriteConfigResponse>> {
        self.write_config_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait NamespaceService {
    fn read_config(&mut self, ctx: ::grpcio::RpcContext, req: super::namespace_service::ReadConfigRequest, sink: ::grpcio::UnarySink<super::namespace_service::ReadConfigResponse>);
    fn write_config(&mut self, ctx: ::grpcio::RpcContext, req: super::namespace_service::WriteConfigRequest, sink: ::grpcio::UnarySink<super::namespace_service::WriteConfigResponse>);
}

pub fn create_namespace_service<S: NamespaceService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_NAMESPACE_SERVICE_READ_CONFIG, move |ctx, req, resp| {
        instance.read_config(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_NAMESPACE_SERVICE_WRITE_CONFIG, move |ctx, req, resp| {
        instance.write_config(ctx, req, resp)
    });
    builder.build()
}
