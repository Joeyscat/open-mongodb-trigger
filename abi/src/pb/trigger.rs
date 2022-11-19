/// Database trigger configuration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseConfig {
    /// The uri of the MongoDB data source.
    #[prost(string, tag = "1")]
    pub data_source: ::prost::alloc::string::String,
    /// The name of the MongoDB database that contains the watched collection.
    #[prost(string, tag = "2")]
    pub database: ::prost::alloc::string::String,
    /// The name of the collection that the trigger watches.
    #[prost(string, tag = "3")]
    pub collection: ::prost::alloc::string::String,
    /// A list of one or more database operation types that cause the trigger to
    /// fire.
    #[prost(enumeration = "DatabaseOperationType", repeated, tag = "4")]
    pub operation_types: ::prost::alloc::vec::Vec<i32>,
}
/// Authentication trigger configuration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationConfig {}
/// Scheduled trigger configuration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduledConfig {}
/// Core trigger object. Contains all the information for a trigger if
/// ListenResponse op is DELETE, only id will be populated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trigger {
    /// unique id for the trigger, if put into CreateRequest, id should be
    /// empty
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// user id for the trigger
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    /// The trigger name. This may be at most 64 characters long and can only
    /// contain ASCII letters, numbers, underscores, and hyphens.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// The trigger type. The value of this field determines the exact
    /// configuration file schema.
    #[prost(enumeration = "TriggerType", tag = "4")]
    pub trigger_type: i32,
    /// The Handle Function that the trigger executes whenever it
    /// fires.
    #[prost(string, tag = "8")]
    pub function_id: ::prost::alloc::string::String,
    /// If false, the trigger will not listen for any events and will not fire
    #[prost(bool, tag = "9")]
    pub enabled: bool,
    /// A document with fields that map to additional configuration options for
    /// the trigger. The exact configuration fields depend on the trigger type
    #[prost(oneof = "trigger::TriggerConfig", tags = "5, 6, 7")]
    pub trigger_config: ::core::option::Option<trigger::TriggerConfig>,
}
/// Nested message and enum types in `Trigger`.
pub mod trigger {
    /// A document with fields that map to additional configuration options for
    /// the trigger. The exact configuration fields depend on the trigger type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TriggerConfig {
        #[prost(message, tag = "5")]
        Database(super::DatabaseConfig),
        #[prost(message, tag = "6")]
        Authentication(super::AuthenticationConfig),
        #[prost(message, tag = "7")]
        Scheduled(super::ScheduledConfig),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResponse {
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub function_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableResponse {
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableResponse {
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteResponse {
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerQuery {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub function_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub trigger_type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub data_source: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub database: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub collection: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    #[prost(message, optional, tag = "1")]
    pub query: ::core::option::Option<TriggerQuery>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
}
/// The trigger type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TriggerType {
    Unknown = 0,
    Database = 1,
    Authentication = 2,
    Scheduled = 3,
}
impl TriggerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TriggerType::Unknown => "TRIGGER_TYPE_UNKNOWN",
            TriggerType::Database => "TRIGGER_TYPE_DATABASE",
            TriggerType::Authentication => "TRIGGER_TYPE_AUTHENTICATION",
            TriggerType::Scheduled => "TRIGGER_TYPE_SCHEDULED",
        }
    }
}
/// Database operation type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseOperationType {
    Unknown = 0,
    Insert = 1,
    Update = 2,
    Replace = 3,
    Delete = 4,
}
impl DatabaseOperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DatabaseOperationType::Unknown => "DATABASE_OPERATION_TYPE_UNKNOWN",
            DatabaseOperationType::Insert => "DATABASE_OPERATION_TYPE_INSERT",
            DatabaseOperationType::Update => "DATABASE_OPERATION_TYPE_UPDATE",
            DatabaseOperationType::Replace => "DATABASE_OPERATION_TYPE_REPLACE",
            DatabaseOperationType::Delete => "DATABASE_OPERATION_TYPE_DELETE",
        }
    }
}
/// Generated client implementations.
pub mod trigger_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Trigger service
    #[derive(Debug, Clone)]
    pub struct TriggerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TriggerServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TriggerServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TriggerServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TriggerServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// create a trigger
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRequest>,
        ) -> Result<tonic::Response<super::CreateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/trigger.TriggerService/create");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// update the trigger
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRequest>,
        ) -> Result<tonic::Response<super::UpdateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/trigger.TriggerService/update");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// enable the trigger
        pub async fn enable(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableRequest>,
        ) -> Result<tonic::Response<super::EnableResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/trigger.TriggerService/enable");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// disable the trigger
        pub async fn disable(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableRequest>,
        ) -> Result<tonic::Response<super::DisableResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/trigger.TriggerService/disable");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// delete the trigger
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRequest>,
        ) -> Result<tonic::Response<super::DeleteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/trigger.TriggerService/delete");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// get the trigger
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/trigger.TriggerService/get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::Trigger>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/trigger.TriggerService/query");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
/// Generated server implementations.
pub mod trigger_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with TriggerServiceServer.
    #[async_trait]
    pub trait TriggerService: Send + Sync + 'static {
        /// create a trigger
        async fn create(
            &self,
            request: tonic::Request<super::CreateRequest>,
        ) -> Result<tonic::Response<super::CreateResponse>, tonic::Status>;
        /// update the trigger
        async fn update(
            &self,
            request: tonic::Request<super::UpdateRequest>,
        ) -> Result<tonic::Response<super::UpdateResponse>, tonic::Status>;
        /// enable the trigger
        async fn enable(
            &self,
            request: tonic::Request<super::EnableRequest>,
        ) -> Result<tonic::Response<super::EnableResponse>, tonic::Status>;
        /// disable the trigger
        async fn disable(
            &self,
            request: tonic::Request<super::DisableRequest>,
        ) -> Result<tonic::Response<super::DisableResponse>, tonic::Status>;
        /// delete the trigger
        async fn delete(
            &self,
            request: tonic::Request<super::DeleteRequest>,
        ) -> Result<tonic::Response<super::DeleteResponse>, tonic::Status>;
        /// get the trigger
        async fn get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status>;
        ///Server streaming response type for the query method.
        type queryStream: futures_core::Stream<Item = Result<super::Trigger, tonic::Status>>
            + Send
            + 'static;
        async fn query(
            &self,
            request: tonic::Request<super::QueryRequest>,
        ) -> Result<tonic::Response<Self::queryStream>, tonic::Status>;
    }
    /// Trigger service
    #[derive(Debug)]
    pub struct TriggerServiceServer<T: TriggerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TriggerService> TriggerServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TriggerServiceServer<T>
    where
        T: TriggerService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/trigger.TriggerService/create" => {
                    #[allow(non_camel_case_types)]
                    struct createSvc<T: TriggerService>(pub Arc<T>);
                    impl<T: TriggerService> tonic::server::UnaryService<super::CreateRequest> for createSvc<T> {
                        type Response = super::CreateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = createSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/trigger.TriggerService/update" => {
                    #[allow(non_camel_case_types)]
                    struct updateSvc<T: TriggerService>(pub Arc<T>);
                    impl<T: TriggerService> tonic::server::UnaryService<super::UpdateRequest> for updateSvc<T> {
                        type Response = super::UpdateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/trigger.TriggerService/enable" => {
                    #[allow(non_camel_case_types)]
                    struct enableSvc<T: TriggerService>(pub Arc<T>);
                    impl<T: TriggerService> tonic::server::UnaryService<super::EnableRequest> for enableSvc<T> {
                        type Response = super::EnableResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EnableRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).enable(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = enableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/trigger.TriggerService/disable" => {
                    #[allow(non_camel_case_types)]
                    struct disableSvc<T: TriggerService>(pub Arc<T>);
                    impl<T: TriggerService> tonic::server::UnaryService<super::DisableRequest> for disableSvc<T> {
                        type Response = super::DisableResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisableRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).disable(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = disableSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/trigger.TriggerService/delete" => {
                    #[allow(non_camel_case_types)]
                    struct deleteSvc<T: TriggerService>(pub Arc<T>);
                    impl<T: TriggerService> tonic::server::UnaryService<super::DeleteRequest> for deleteSvc<T> {
                        type Response = super::DeleteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = deleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/trigger.TriggerService/get" => {
                    #[allow(non_camel_case_types)]
                    struct getSvc<T: TriggerService>(pub Arc<T>);
                    impl<T: TriggerService> tonic::server::UnaryService<super::GetRequest> for getSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/trigger.TriggerService/query" => {
                    #[allow(non_camel_case_types)]
                    struct querySvc<T: TriggerService>(pub Arc<T>);
                    impl<T: TriggerService>
                        tonic::server::ServerStreamingService<super::QueryRequest> for querySvc<T>
                    {
                        type Response = super::Trigger;
                        type ResponseStream = T::queryStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).query(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = querySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: TriggerService> Clone for TriggerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TriggerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TriggerService> tonic::server::NamedService for TriggerServiceServer<T> {
        const NAME: &'static str = "trigger.TriggerService";
    }
}
