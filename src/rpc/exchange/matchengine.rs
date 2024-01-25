#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceQueryRequest {
    /// optional
    #[prost(string, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceQueryResponse {
    #[prost(message, repeated, tag = "1")]
    pub balances: ::prost::alloc::vec::Vec<balance_query_response::AssetBalance>,
}
/// Nested message and enum types in `BalanceQueryResponse`.
pub mod balance_query_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AssetBalance {
        #[prost(string, tag = "1")]
        pub asset_id: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub available: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub frozen: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdateRequest {
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub asset: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub business: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub business_id: u64,
    #[prost(string, tag = "5")]
    pub delta: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub detail: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceUpdateResponse {}
/// repeated string assets = 1;
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetListRequest {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetListResponse {
    #[prost(message, repeated, tag = "1")]
    pub asset_lists: ::prost::alloc::vec::Vec<asset_list_response::AssetInfo>,
}
/// Nested message and enum types in `AssetListResponse`.
pub mod asset_list_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AssetInfo {
        #[prost(string, tag = "1")]
        pub symbol: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub name: ::prost::alloc::string::String,
        #[prost(uint32, tag = "3")]
        pub precision: u32,
    }
}
///
/// internal?
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSummaryRequest {
    #[prost(string, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSummaryResponse {
    #[prost(message, repeated, tag = "1")]
    pub asset_summaries: ::prost::alloc::vec::Vec<
        asset_summary_response::AssetSummaryInfo,
    >,
}
/// Nested message and enum types in `AssetSummaryResponse`.
pub mod asset_summary_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AssetSummaryInfo {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub total_balance: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub available_count: i32,
        #[prost(string, tag = "4")]
        pub available_balance: ::prost::alloc::string::String,
        #[prost(int32, tag = "5")]
        pub frozen_count: i32,
        #[prost(string, tag = "6")]
        pub frozen_balance: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderPutRequest {
    #[prost(string, tag = "1")]
    pub market: ::prost::alloc::string::String,
    #[prost(enumeration = "OrderSide", tag = "2")]
    pub order_side: i32,
    #[prost(enumeration = "OrderType", tag = "3")]
    pub order_type: i32,
    /// always amount for base, even for market bid
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    /// should be empty or zero for market order
    #[prost(string, tag = "5")]
    pub price: ::prost::alloc::string::String,
    /// onyl valid for market bid order
    #[prost(string, tag = "6")]
    pub quote_limit: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub taker_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub maker_fee: ::prost::alloc::string::String,
    /// Ensures an Limit order is only subject to Maker Fees
    #[prost(bool, tag = "9")]
    pub post_only: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderInfo {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub market: ::prost::alloc::string::String,
    #[prost(enumeration = "OrderSide", tag = "3")]
    pub order_side: i32,
    #[prost(enumeration = "OrderType", tag = "4")]
    pub order_type: i32,
    #[prost(int64, tag = "5")]
    pub create_time: i64,
    #[prost(int64, tag = "6")]
    pub update_time: i64,
    #[prost(string, tag = "7")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub taker_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub maker_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub remain: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub finished_base: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub finished_quote: ::prost::alloc::string::String,
    #[prost(string, tag = "14")]
    pub finished_fee: ::prost::alloc::string::String,
    #[prost(bool, tag = "15")]
    pub post_only: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderQueryRequest {
    #[prost(string, tag = "1")]
    pub market: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub offset: i32,
    #[prost(int32, tag = "3")]
    pub limit: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderQueryResponse {
    #[prost(int32, tag = "1")]
    pub offset: i32,
    #[prost(int32, tag = "2")]
    pub limit: i32,
    #[prost(int32, tag = "3")]
    pub total: i32,
    #[prost(message, repeated, tag = "4")]
    pub orders: ::prost::alloc::vec::Vec<OrderInfo>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderCancelRequest {
    #[prost(string, tag = "1")]
    pub market: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub order_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderCancelAllRequest {
    #[prost(string, tag = "1")]
    pub market: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderCancelAllResponse {
    #[prost(uint32, tag = "1")]
    pub total: u32,
}
/// why not both side
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookRequest {
    #[prost(string, tag = "1")]
    pub market: ::prost::alloc::string::String,
    #[prost(enumeration = "OrderSide", tag = "2")]
    pub side: i32,
    #[prost(int32, tag = "3")]
    pub offset: i32,
    #[prost(int32, tag = "4")]
    pub limit: i32,
}
/// strange api
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookResponse {
    #[prost(int32, tag = "1")]
    pub offset: i32,
    #[prost(int32, tag = "2")]
    pub limit: i32,
    #[prost(uint64, tag = "3")]
    pub total: u64,
    #[prost(message, repeated, tag = "4")]
    pub orders: ::prost::alloc::vec::Vec<OrderInfo>,
}
/// with cache
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookDepthRequest {
    #[prost(string, tag = "1")]
    pub market: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub limit: i32,
    #[prost(string, tag = "3")]
    pub interval: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookDepthResponse {
    #[prost(message, repeated, tag = "1")]
    pub asks: ::prost::alloc::vec::Vec<order_book_depth_response::PriceInfo>,
    #[prost(message, repeated, tag = "2")]
    pub bids: ::prost::alloc::vec::Vec<order_book_depth_response::PriceInfo>,
}
/// Nested message and enum types in `OrderBookDepthResponse`.
pub mod order_book_depth_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PriceInfo {
        #[prost(string, tag = "1")]
        pub price: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub amount: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderDetailRequest {
    #[prost(string, tag = "1")]
    pub market: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub order_id: u64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOrderPutRequest {
    #[prost(string, tag = "1")]
    pub market: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub reset: bool,
    #[prost(message, repeated, tag = "3")]
    pub orders: ::prost::alloc::vec::Vec<OrderPutRequest>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOrderPutResponse {
    #[prost(enumeration = "ResultCode", tag = "1")]
    pub result_code: i32,
    #[prost(string, tag = "2")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag = "3")]
    pub order_ids: ::prost::alloc::vec::Vec<u64>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketListRequest {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketListResponse {
    #[prost(message, repeated, tag = "1")]
    pub markets: ::prost::alloc::vec::Vec<market_list_response::MarketInfo>,
}
/// Nested message and enum types in `MarketListResponse`.
pub mod market_list_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MarketInfo {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// base
        #[prost(string, tag = "2")]
        pub base: ::prost::alloc::string::String,
        /// quote
        #[prost(string, tag = "3")]
        pub quote: ::prost::alloc::string::String,
        #[prost(uint32, tag = "4")]
        pub fee_precision: u32,
        #[prost(uint32, tag = "5")]
        pub amount_precision: u32,
        #[prost(uint32, tag = "6")]
        pub price_precision: u32,
        #[prost(string, tag = "7")]
        pub min_amount: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketSummaryRequest {
    #[prost(string, repeated, tag = "1")]
    pub markets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketSummaryResponse {
    #[prost(message, repeated, tag = "1")]
    pub market_summaries: ::prost::alloc::vec::Vec<
        market_summary_response::MarketSummary,
    >,
}
/// Nested message and enum types in `MarketSummaryResponse`.
pub mod market_summary_response {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MarketSummary {
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub ask_count: i32,
        #[prost(string, tag = "3")]
        pub ask_amount: ::prost::alloc::string::String,
        #[prost(int32, tag = "4")]
        pub bid_count: i32,
        #[prost(string, tag = "5")]
        pub bid_amount: ::prost::alloc::string::String,
        #[prost(uint64, tag = "6")]
        pub trade_count: u64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReloadMarketsRequest {
    #[prost(bool, tag = "1")]
    pub from_scratch: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleSuccessResponse {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugDumpRequest {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugDumpResponse {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugResetRequest {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugResetResponse {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugReloadRequest {}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugReloadResponse {}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderSide {
    Ask = 0,
    Bid = 1,
}
impl OrderSide {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderSide::Ask => "ASK",
            OrderSide::Bid => "BID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASK" => Some(Self::Ask),
            "BID" => Some(Self::Bid),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    Limit = 0,
    Market = 1,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Limit => "LIMIT",
            OrderType::Market => "MARKET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIMIT" => Some(Self::Limit),
            "MARKET" => Some(Self::Market),
            _ => None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResultCode {
    Success = 0,
    InternalError = 1,
}
impl ResultCode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResultCode::Success => "SUCCESS",
            ResultCode::InternalError => "INTERNAL_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUCCESS" => Some(Self::Success),
            "INTERNAL_ERROR" => Some(Self::InternalError),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod matchengine_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MatchengineClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MatchengineClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MatchengineClient<T>
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
        ) -> MatchengineClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MatchengineClient::new(InterceptedService::new(inner, interceptor))
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn balance_query(
            &mut self,
            request: impl tonic::IntoRequest<super::BalanceQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BalanceQueryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/BalanceQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "BalanceQuery"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn balance_update(
            &mut self,
            request: impl tonic::IntoRequest<super::BalanceUpdateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BalanceUpdateResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/BalanceUpdate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "BalanceUpdate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn asset_list(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AssetListResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/AssetList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "AssetList"));
            self.inner.unary(req, path, codec).await
        }
        /// rpc AssetSummary(AssetSummaryRequest) returns (AssetSummaryResponse) {}
        pub async fn order_put(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderPutRequest>,
        ) -> std::result::Result<tonic::Response<super::OrderInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/OrderPut",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "OrderPut"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn batch_order_put(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchOrderPutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchOrderPutResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/BatchOrderPut",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "BatchOrderPut"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn order_query(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderQueryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/OrderQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "OrderQuery"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn order_cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderCancelRequest>,
        ) -> std::result::Result<tonic::Response<super::OrderInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/OrderCancel",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "OrderCancel"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn order_cancel_all(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderCancelAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderCancelAllResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/OrderCancelAll",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "OrderCancelAll"));
            self.inner.unary(req, path, codec).await
        }
        /// rpc OrderBook(OrderBookRequest) returns (OrderBookResponse) {}
        pub async fn order_book_depth(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderBookDepthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderBookDepthResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/OrderBookDepth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "OrderBookDepth"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn order_detail(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderDetailRequest>,
        ) -> std::result::Result<tonic::Response<super::OrderInfo>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/OrderDetail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "OrderDetail"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_list(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarketListResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/MarketList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "MarketList"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn reload_markets(
            &mut self,
            request: impl tonic::IntoRequest<super::ReloadMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SimpleSuccessResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/ReloadMarkets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "ReloadMarkets"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn market_summary(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketSummaryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarketSummaryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/MarketSummary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "MarketSummary"));
            self.inner.unary(req, path, codec).await
        }
        /// Used only in development
        pub async fn debug_dump(
            &mut self,
            request: impl tonic::IntoRequest<super::DebugDumpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DebugDumpResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/DebugDump",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "DebugDump"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn debug_reset(
            &mut self,
            request: impl tonic::IntoRequest<super::DebugResetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DebugResetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/DebugReset",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "DebugReset"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn debug_reload(
            &mut self,
            request: impl tonic::IntoRequest<super::DebugReloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DebugReloadResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/matchengine.Matchengine/DebugReload",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("matchengine.Matchengine", "DebugReload"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod matchengine_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MatchengineServer.
    #[async_trait]
    pub trait Matchengine: Send + Sync + 'static {
        async fn balance_query(
            &self,
            request: tonic::Request<super::BalanceQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BalanceQueryResponse>,
            tonic::Status,
        >;
        async fn balance_update(
            &self,
            request: tonic::Request<super::BalanceUpdateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BalanceUpdateResponse>,
            tonic::Status,
        >;
        async fn asset_list(
            &self,
            request: tonic::Request<super::AssetListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AssetListResponse>,
            tonic::Status,
        >;
        /// rpc AssetSummary(AssetSummaryRequest) returns (AssetSummaryResponse) {}
        async fn order_put(
            &self,
            request: tonic::Request<super::OrderPutRequest>,
        ) -> std::result::Result<tonic::Response<super::OrderInfo>, tonic::Status>;
        async fn batch_order_put(
            &self,
            request: tonic::Request<super::BatchOrderPutRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BatchOrderPutResponse>,
            tonic::Status,
        >;
        async fn order_query(
            &self,
            request: tonic::Request<super::OrderQueryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderQueryResponse>,
            tonic::Status,
        >;
        async fn order_cancel(
            &self,
            request: tonic::Request<super::OrderCancelRequest>,
        ) -> std::result::Result<tonic::Response<super::OrderInfo>, tonic::Status>;
        async fn order_cancel_all(
            &self,
            request: tonic::Request<super::OrderCancelAllRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderCancelAllResponse>,
            tonic::Status,
        >;
        /// rpc OrderBook(OrderBookRequest) returns (OrderBookResponse) {}
        async fn order_book_depth(
            &self,
            request: tonic::Request<super::OrderBookDepthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OrderBookDepthResponse>,
            tonic::Status,
        >;
        async fn order_detail(
            &self,
            request: tonic::Request<super::OrderDetailRequest>,
        ) -> std::result::Result<tonic::Response<super::OrderInfo>, tonic::Status>;
        async fn market_list(
            &self,
            request: tonic::Request<super::MarketListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarketListResponse>,
            tonic::Status,
        >;
        async fn reload_markets(
            &self,
            request: tonic::Request<super::ReloadMarketsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SimpleSuccessResponse>,
            tonic::Status,
        >;
        async fn market_summary(
            &self,
            request: tonic::Request<super::MarketSummaryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarketSummaryResponse>,
            tonic::Status,
        >;
        /// Used only in development
        async fn debug_dump(
            &self,
            request: tonic::Request<super::DebugDumpRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DebugDumpResponse>,
            tonic::Status,
        >;
        async fn debug_reset(
            &self,
            request: tonic::Request<super::DebugResetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DebugResetResponse>,
            tonic::Status,
        >;
        async fn debug_reload(
            &self,
            request: tonic::Request<super::DebugReloadRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DebugReloadResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MatchengineServer<T: Matchengine> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Matchengine> MatchengineServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MatchengineServer<T>
    where
        T: Matchengine,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/matchengine.Matchengine/BalanceQuery" => {
                    #[allow(non_camel_case_types)]
                    struct BalanceQuerySvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::BalanceQueryRequest>
                    for BalanceQuerySvc<T> {
                        type Response = super::BalanceQueryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BalanceQueryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::balance_query(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BalanceQuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/BalanceUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct BalanceUpdateSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::BalanceUpdateRequest>
                    for BalanceUpdateSvc<T> {
                        type Response = super::BalanceUpdateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BalanceUpdateRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::balance_update(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BalanceUpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/AssetList" => {
                    #[allow(non_camel_case_types)]
                    struct AssetListSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::AssetListRequest>
                    for AssetListSvc<T> {
                        type Response = super::AssetListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AssetListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::asset_list(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AssetListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/OrderPut" => {
                    #[allow(non_camel_case_types)]
                    struct OrderPutSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::OrderPutRequest>
                    for OrderPutSvc<T> {
                        type Response = super::OrderInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderPutRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::order_put(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderPutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/BatchOrderPut" => {
                    #[allow(non_camel_case_types)]
                    struct BatchOrderPutSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::BatchOrderPutRequest>
                    for BatchOrderPutSvc<T> {
                        type Response = super::BatchOrderPutResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchOrderPutRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::batch_order_put(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BatchOrderPutSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/OrderQuery" => {
                    #[allow(non_camel_case_types)]
                    struct OrderQuerySvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::OrderQueryRequest>
                    for OrderQuerySvc<T> {
                        type Response = super::OrderQueryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderQueryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::order_query(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderQuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/OrderCancel" => {
                    #[allow(non_camel_case_types)]
                    struct OrderCancelSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::OrderCancelRequest>
                    for OrderCancelSvc<T> {
                        type Response = super::OrderInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderCancelRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::order_cancel(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderCancelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/OrderCancelAll" => {
                    #[allow(non_camel_case_types)]
                    struct OrderCancelAllSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::OrderCancelAllRequest>
                    for OrderCancelAllSvc<T> {
                        type Response = super::OrderCancelAllResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderCancelAllRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::order_cancel_all(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderCancelAllSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/OrderBookDepth" => {
                    #[allow(non_camel_case_types)]
                    struct OrderBookDepthSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::OrderBookDepthRequest>
                    for OrderBookDepthSvc<T> {
                        type Response = super::OrderBookDepthResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderBookDepthRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::order_book_depth(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderBookDepthSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/OrderDetail" => {
                    #[allow(non_camel_case_types)]
                    struct OrderDetailSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::OrderDetailRequest>
                    for OrderDetailSvc<T> {
                        type Response = super::OrderInfo;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OrderDetailRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::order_detail(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OrderDetailSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/MarketList" => {
                    #[allow(non_camel_case_types)]
                    struct MarketListSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::MarketListRequest>
                    for MarketListSvc<T> {
                        type Response = super::MarketListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketListRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::market_list(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarketListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/ReloadMarkets" => {
                    #[allow(non_camel_case_types)]
                    struct ReloadMarketsSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::ReloadMarketsRequest>
                    for ReloadMarketsSvc<T> {
                        type Response = super::SimpleSuccessResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReloadMarketsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::reload_markets(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReloadMarketsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/MarketSummary" => {
                    #[allow(non_camel_case_types)]
                    struct MarketSummarySvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::MarketSummaryRequest>
                    for MarketSummarySvc<T> {
                        type Response = super::MarketSummaryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarketSummaryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::market_summary(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarketSummarySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/DebugDump" => {
                    #[allow(non_camel_case_types)]
                    struct DebugDumpSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::DebugDumpRequest>
                    for DebugDumpSvc<T> {
                        type Response = super::DebugDumpResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DebugDumpRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::debug_dump(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DebugDumpSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/DebugReset" => {
                    #[allow(non_camel_case_types)]
                    struct DebugResetSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::DebugResetRequest>
                    for DebugResetSvc<T> {
                        type Response = super::DebugResetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DebugResetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::debug_reset(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DebugResetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/matchengine.Matchengine/DebugReload" => {
                    #[allow(non_camel_case_types)]
                    struct DebugReloadSvc<T: Matchengine>(pub Arc<T>);
                    impl<
                        T: Matchengine,
                    > tonic::server::UnaryService<super::DebugReloadRequest>
                    for DebugReloadSvc<T> {
                        type Response = super::DebugReloadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DebugReloadRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as Matchengine>::debug_reload(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DebugReloadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Matchengine> Clone for MatchengineServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Matchengine> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Matchengine> tonic::server::NamedService for MatchengineServer<T> {
        const NAME: &'static str = "matchengine.Matchengine";
    }
}
