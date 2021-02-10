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

const METHOD_ACL_SERVICE_READ: ::grpcio::Method<super::acl_service::ReadRequest, super::acl_service::ReadResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/ACLService/Read",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACL_SERVICE_WRITE: ::grpcio::Method<super::acl_service::WriteRequest, super::acl_service::WriteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/ACLService/Write",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACL_SERVICE_CHECK: ::grpcio::Method<super::acl_service::CheckRequest, super::acl_service::CheckResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/ACLService/Check",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACL_SERVICE_CONTENT_CHANGE_CHECK: ::grpcio::Method<super::acl_service::ContentChangeCheckRequest, super::acl_service::CheckResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/ACLService/ContentChangeCheck",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_ACL_SERVICE_EXPAND: ::grpcio::Method<super::acl_service::ExpandRequest, super::acl_service::ExpandResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/ACLService/Expand",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct AclServiceClient {
    client: ::grpcio::Client,
}

impl AclServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        AclServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn read_opt(&self, req: &super::acl_service::ReadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::acl_service::ReadResponse> {
        self.client.unary_call(&METHOD_ACL_SERVICE_READ, req, opt)
    }

    pub fn read(&self, req: &super::acl_service::ReadRequest) -> ::grpcio::Result<super::acl_service::ReadResponse> {
        self.read_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_async_opt(&self, req: &super::acl_service::ReadRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::acl_service::ReadResponse>> {
        self.client.unary_call_async(&METHOD_ACL_SERVICE_READ, req, opt)
    }

    pub fn read_async(&self, req: &super::acl_service::ReadRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::acl_service::ReadResponse>> {
        self.read_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_opt(&self, req: &super::acl_service::WriteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::acl_service::WriteResponse> {
        self.client.unary_call(&METHOD_ACL_SERVICE_WRITE, req, opt)
    }

    pub fn write(&self, req: &super::acl_service::WriteRequest) -> ::grpcio::Result<super::acl_service::WriteResponse> {
        self.write_opt(req, ::grpcio::CallOption::default())
    }

    pub fn write_async_opt(&self, req: &super::acl_service::WriteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::acl_service::WriteResponse>> {
        self.client.unary_call_async(&METHOD_ACL_SERVICE_WRITE, req, opt)
    }

    pub fn write_async(&self, req: &super::acl_service::WriteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::acl_service::WriteResponse>> {
        self.write_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_opt(&self, req: &super::acl_service::CheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::acl_service::CheckResponse> {
        self.client.unary_call(&METHOD_ACL_SERVICE_CHECK, req, opt)
    }

    pub fn check(&self, req: &super::acl_service::CheckRequest) -> ::grpcio::Result<super::acl_service::CheckResponse> {
        self.check_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_async_opt(&self, req: &super::acl_service::CheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::acl_service::CheckResponse>> {
        self.client.unary_call_async(&METHOD_ACL_SERVICE_CHECK, req, opt)
    }

    pub fn check_async(&self, req: &super::acl_service::CheckRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::acl_service::CheckResponse>> {
        self.check_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn content_change_check_opt(&self, req: &super::acl_service::ContentChangeCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::acl_service::CheckResponse> {
        self.client.unary_call(&METHOD_ACL_SERVICE_CONTENT_CHANGE_CHECK, req, opt)
    }

    pub fn content_change_check(&self, req: &super::acl_service::ContentChangeCheckRequest) -> ::grpcio::Result<super::acl_service::CheckResponse> {
        self.content_change_check_opt(req, ::grpcio::CallOption::default())
    }

    pub fn content_change_check_async_opt(&self, req: &super::acl_service::ContentChangeCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::acl_service::CheckResponse>> {
        self.client.unary_call_async(&METHOD_ACL_SERVICE_CONTENT_CHANGE_CHECK, req, opt)
    }

    pub fn content_change_check_async(&self, req: &super::acl_service::ContentChangeCheckRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::acl_service::CheckResponse>> {
        self.content_change_check_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn expand_opt(&self, req: &super::acl_service::ExpandRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::acl_service::ExpandResponse> {
        self.client.unary_call(&METHOD_ACL_SERVICE_EXPAND, req, opt)
    }

    pub fn expand(&self, req: &super::acl_service::ExpandRequest) -> ::grpcio::Result<super::acl_service::ExpandResponse> {
        self.expand_opt(req, ::grpcio::CallOption::default())
    }

    pub fn expand_async_opt(&self, req: &super::acl_service::ExpandRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::acl_service::ExpandResponse>> {
        self.client.unary_call_async(&METHOD_ACL_SERVICE_EXPAND, req, opt)
    }

    pub fn expand_async(&self, req: &super::acl_service::ExpandRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::acl_service::ExpandResponse>> {
        self.expand_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait AclService {
    fn read(&mut self, ctx: ::grpcio::RpcContext, req: super::acl_service::ReadRequest, sink: ::grpcio::UnarySink<super::acl_service::ReadResponse>);
    fn write(&mut self, ctx: ::grpcio::RpcContext, req: super::acl_service::WriteRequest, sink: ::grpcio::UnarySink<super::acl_service::WriteResponse>);
    fn check(&mut self, ctx: ::grpcio::RpcContext, req: super::acl_service::CheckRequest, sink: ::grpcio::UnarySink<super::acl_service::CheckResponse>);
    fn content_change_check(&mut self, ctx: ::grpcio::RpcContext, req: super::acl_service::ContentChangeCheckRequest, sink: ::grpcio::UnarySink<super::acl_service::CheckResponse>);
    fn expand(&mut self, ctx: ::grpcio::RpcContext, req: super::acl_service::ExpandRequest, sink: ::grpcio::UnarySink<super::acl_service::ExpandResponse>);
}

pub fn create_acl_service<S: AclService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACL_SERVICE_READ, move |ctx, req, resp| {
        instance.read(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACL_SERVICE_WRITE, move |ctx, req, resp| {
        instance.write(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACL_SERVICE_CHECK, move |ctx, req, resp| {
        instance.check(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_ACL_SERVICE_CONTENT_CHANGE_CHECK, move |ctx, req, resp| {
        instance.content_change_check(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_ACL_SERVICE_EXPAND, move |ctx, req, resp| {
        instance.expand(ctx, req, resp)
    });
    builder.build()
}
