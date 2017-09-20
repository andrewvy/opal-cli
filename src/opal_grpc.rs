// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

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


// interface

pub trait PInternal {
    fn get_wallet(&self, o: ::grpc::RequestOptions, p: super::opal::PEmpty) -> ::grpc::SingleResponse<super::opal::PWallet>;

    fn send_transaction(&self, o: ::grpc::RequestOptions, p: super::opal::PSendTransactionRequest) -> ::grpc::SingleResponse<super::opal::PSendTransactionResponse>;
}

// client

pub struct PInternalClient {
    grpc_client: ::grpc::Client,
    method_GetWallet: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::opal::PEmpty, super::opal::PWallet>>,
    method_SendTransaction: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::opal::PSendTransactionRequest, super::opal::PSendTransactionResponse>>,
}

impl PInternalClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        PInternalClient {
            grpc_client: grpc_client,
            method_GetWallet: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/PInternal/GetWallet".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_SendTransaction: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/PInternal/SendTransaction".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            PInternalClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            PInternalClient::with_client(c)
        })
    }
}

impl PInternal for PInternalClient {
    fn get_wallet(&self, o: ::grpc::RequestOptions, p: super::opal::PEmpty) -> ::grpc::SingleResponse<super::opal::PWallet> {
        self.grpc_client.call_unary(o, p, self.method_GetWallet.clone())
    }

    fn send_transaction(&self, o: ::grpc::RequestOptions, p: super::opal::PSendTransactionRequest) -> ::grpc::SingleResponse<super::opal::PSendTransactionResponse> {
        self.grpc_client.call_unary(o, p, self.method_SendTransaction.clone())
    }
}

// server

pub struct PInternalServer;


impl PInternalServer {
    pub fn new_service_def<H : PInternal + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/PInternal",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/PInternal/GetWallet".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_wallet(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/PInternal/SendTransaction".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.send_transaction(o, p))
                    },
                ),
            ],
        )
    }
}
