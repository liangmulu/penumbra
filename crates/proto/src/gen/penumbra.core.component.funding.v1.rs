// This file is @generated by prost-build.
/// Request if a nullifier is already used in a specific epoch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LqtCheckNullifierRequest {
    /// The epoch index in which to check for nullifier usage.
    #[prost(uint64, tag = "1")]
    pub epoch_index: u64,
    /// The nullifier whose voting status is being queried.
    #[prost(message, optional, tag = "2")]
    pub nullifier: ::core::option::Option<super::super::sct::v1::Nullifier>,
}
impl ::prost::Name for LqtCheckNullifierRequest {
    const NAME: &'static str = "LqtCheckNullifierRequest";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.LqtCheckNullifierRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.LqtCheckNullifierRequest".into()
    }
}
/// Response indicating whether the nullifier was already used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LqtCheckNullifierResponse {
    /// The transaction ID of the vote, if the nullifier has been used.
    /// This field is always present if `already_voted` is true.
    #[prost(message, optional, tag = "1")]
    pub transaction: ::core::option::Option<
        super::super::super::txhash::v1::TransactionId,
    >,
    /// Indicates whether the nullifier has already been used for voting in the given epoch.
    #[prost(bool, tag = "2")]
    pub already_voted: bool,
    /// The epoch index in which the nullifier was checked.
    #[prost(uint64, tag = "3")]
    pub epoch_index: u64,
}
impl ::prost::Name for LqtCheckNullifierResponse {
    const NAME: &'static str = "LqtCheckNullifierResponse";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.LqtCheckNullifierResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.LqtCheckNullifierResponse".into()
    }
}
/// Funding component configuration data.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct FundingParameters {
    /// The parameters governing the funding of the liquidity tournament.
    #[prost(message, optional, tag = "1")]
    pub liquidity_tournament: ::core::option::Option<
        funding_parameters::LiquidityTournament,
    >,
}
/// Nested message and enum types in `FundingParameters`.
pub mod funding_parameters {
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct LiquidityTournament {
        /// The fraction of gauge votes that an asset must pass to get any rewards.
        ///
        /// Takes a value in \[0, 100\].
        #[prost(uint64, tag = "1")]
        pub gauge_threshold_percent: u64,
        /// The maximum number of liquidity positions that can receive rewards.
        ///
        /// This avoids potential DoS vectors with processing a large number of small positions.
        #[prost(uint64, tag = "2")]
        pub max_positions: u64,
        /// The maximum number of delegators that can be rewarded.
        ///
        /// Also avoids potential DoS vectors
        #[prost(uint64, tag = "3")]
        pub max_delegators: u64,
        /// The share of rewards which will go to delegators, opposed with positions.
        ///
        /// Takes a value in \[0, 100\].
        #[prost(uint64, tag = "4")]
        pub delegator_share_percent: u64,
    }
    impl ::prost::Name for LiquidityTournament {
        const NAME: &'static str = "LiquidityTournament";
        const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
        fn full_name() -> ::prost::alloc::string::String {
            "penumbra.core.component.funding.v1.FundingParameters.LiquidityTournament"
                .into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "/penumbra.core.component.funding.v1.FundingParameters.LiquidityTournament"
                .into()
        }
    }
}
impl ::prost::Name for FundingParameters {
    const NAME: &'static str = "FundingParameters";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.FundingParameters".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.FundingParameters".into()
    }
}
/// Genesis data for the funding component.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GenesisContent {
    #[prost(message, optional, tag = "1")]
    pub funding_params: ::core::option::Option<FundingParameters>,
}
impl ::prost::Name for GenesisContent {
    const NAME: &'static str = "GenesisContent";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.GenesisContent".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.GenesisContent".into()
    }
}
/// Indicates that a funding stream reward was paid.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventFundingStreamReward {
    /// The recipient of the funding stream reward.
    /// This is a string value for future extensibility.
    /// Currently it will be either "community-pool"
    /// or an address.
    #[prost(string, tag = "1")]
    pub recipient: ::prost::alloc::string::String,
    /// The epoch for which the reward was paid.
    #[prost(uint64, tag = "2")]
    pub epoch_index: u64,
    /// The amount of the reward, in staking tokens.
    #[prost(message, optional, tag = "3")]
    pub reward_amount: ::core::option::Option<super::super::super::num::v1::Amount>,
}
impl ::prost::Name for EventFundingStreamReward {
    const NAME: &'static str = "EventFundingStreamReward";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.EventFundingStreamReward".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.EventFundingStreamReward".into()
    }
}
/// An action for voting in a liquidity tournament.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionLiquidityTournamentVote {
    /// The effectful data signalling user intent, and the validity of this intent.
    #[prost(message, optional, tag = "1")]
    pub body: ::core::option::Option<LiquidityTournamentVoteBody>,
    /// An authorization from the user over this body.
    #[prost(message, optional, tag = "2")]
    pub auth_sig: ::core::option::Option<
        super::super::super::super::crypto::decaf377_rdsa::v1::SpendAuthSignature,
    >,
    /// A ZK proof that it was correctly constructed from private user state.
    #[prost(message, optional, tag = "3")]
    pub proof: ::core::option::Option<ZkLiquidityTournamentVoteProof>,
}
impl ::prost::Name for ActionLiquidityTournamentVote {
    const NAME: &'static str = "ActionLiquidityTournamentVote";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.ActionLiquidityTournamentVote".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.ActionLiquidityTournamentVote".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidityTournamentVoteBody {
    /// Which asset should be incentivized.
    #[prost(message, optional, tag = "1")]
    pub incentivized: ::core::option::Option<super::super::super::asset::v1::Denom>,
    /// Where to send any rewards for participating in the tournament.
    #[prost(message, optional, tag = "2")]
    pub rewards_recipient: ::core::option::Option<
        super::super::super::keys::v1::Address,
    >,
    /// The start position of the tournament
    #[prost(uint64, tag = "3")]
    pub start_position: u64,
    /// The value being voted with.
    ///
    /// This should be some amount of a validator's delegation token.
    #[prost(message, optional, tag = "4")]
    pub value: ::core::option::Option<super::super::super::asset::v1::Value>,
    /// The nullifier associated with the note being spent.
    #[prost(message, optional, tag = "5")]
    pub nullifier: ::core::option::Option<super::super::sct::v1::Nullifier>,
    /// A randomized verification key with which to check the auth signature.
    #[prost(message, optional, tag = "6")]
    pub rk: ::core::option::Option<
        super::super::super::super::crypto::decaf377_rdsa::v1::SpendVerificationKey,
    >,
}
impl ::prost::Name for LiquidityTournamentVoteBody {
    const NAME: &'static str = "LiquidityTournamentVoteBody";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.LiquidityTournamentVoteBody".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.LiquidityTournamentVoteBody".into()
    }
}
/// The plan associated with a `ActionLiquidityTournamentVote`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionLiquidityTournamentVotePlan {
    /// The asset the user wants to vote for.
    #[prost(message, optional, tag = "1")]
    pub incentivized: ::core::option::Option<super::super::super::asset::v1::Denom>,
    /// Where to send any rewards for participating in the tournament.
    #[prost(message, optional, tag = "2")]
    pub rewards_recipient: ::core::option::Option<
        super::super::super::keys::v1::Address,
    >,
    /// The note containing the staked note used for voting.
    #[prost(message, optional, tag = "3")]
    pub staked_note: ::core::option::Option<super::super::shielded_pool::v1::Note>,
    /// The position of the staked note.
    #[prost(uint64, tag = "4")]
    pub staked_note_position: u64,
    /// The start position of the tournament.
    #[prost(uint64, tag = "5")]
    pub start_position: u64,
    /// Randomizer for proof of spend capability.
    #[prost(bytes = "vec", tag = "6")]
    pub randomizer: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub proof_blinding_r: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "8")]
    pub proof_blinding_s: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ActionLiquidityTournamentVotePlan {
    const NAME: &'static str = "ActionLiquidityTournamentVotePlan";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.ActionLiquidityTournamentVotePlan".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.ActionLiquidityTournamentVotePlan".into()
    }
}
/// A proof of the validity of a liquidity vote, wrt private state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZkLiquidityTournamentVoteProof {
    #[prost(bytes = "vec", tag = "1")]
    pub inner: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for ZkLiquidityTournamentVoteProof {
    const NAME: &'static str = "ZKLiquidityTournamentVoteProof";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.ZKLiquidityTournamentVoteProof".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.ZKLiquidityTournamentVoteProof".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionLiquidityTournamentVoteView {
    #[prost(
        oneof = "action_liquidity_tournament_vote_view::LiquidityTournamentVote",
        tags = "1, 2"
    )]
    pub liquidity_tournament_vote: ::core::option::Option<
        action_liquidity_tournament_vote_view::LiquidityTournamentVote,
    >,
}
/// Nested message and enum types in `ActionLiquidityTournamentVoteView`.
pub mod action_liquidity_tournament_vote_view {
    /// If we initiated the vote, we should know the note that we spent.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Visible {
        #[prost(message, optional, tag = "1")]
        pub vote: ::core::option::Option<super::ActionLiquidityTournamentVote>,
        #[prost(message, optional, tag = "2")]
        pub note: ::core::option::Option<
            super::super::super::shielded_pool::v1::NoteView,
        >,
    }
    impl ::prost::Name for Visible {
        const NAME: &'static str = "Visible";
        const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
        fn full_name() -> ::prost::alloc::string::String {
            "penumbra.core.component.funding.v1.ActionLiquidityTournamentVoteView.Visible"
                .into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "/penumbra.core.component.funding.v1.ActionLiquidityTournamentVoteView.Visible"
                .into()
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Opaque {
        #[prost(message, optional, tag = "1")]
        pub vote: ::core::option::Option<super::ActionLiquidityTournamentVote>,
    }
    impl ::prost::Name for Opaque {
        const NAME: &'static str = "Opaque";
        const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
        fn full_name() -> ::prost::alloc::string::String {
            "penumbra.core.component.funding.v1.ActionLiquidityTournamentVoteView.Opaque"
                .into()
        }
        fn type_url() -> ::prost::alloc::string::String {
            "/penumbra.core.component.funding.v1.ActionLiquidityTournamentVoteView.Opaque"
                .into()
        }
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LiquidityTournamentVote {
        #[prost(message, tag = "1")]
        Visible(Visible),
        #[prost(message, tag = "2")]
        Opaque(Opaque),
    }
}
impl ::prost::Name for ActionLiquidityTournamentVoteView {
    const NAME: &'static str = "ActionLiquidityTournamentVoteView";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.ActionLiquidityTournamentVoteView".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.ActionLiquidityTournamentVoteView".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventLqtDelegatorReward {
    /// The epoch for which the reward was paid.
    #[prost(uint64, tag = "1")]
    pub epoch_index: u64,
    /// The amount of reward, in staking tokens.
    #[prost(message, optional, tag = "2")]
    pub reward_amount: ::core::option::Option<super::super::super::num::v1::Amount>,
    /// The amount of reward, in delegation tokens.
    #[prost(message, optional, tag = "3")]
    pub delegation_tokens: ::core::option::Option<super::super::super::asset::v1::Value>,
    /// The recipient of the reward
    #[prost(message, optional, tag = "4")]
    pub address: ::core::option::Option<super::super::super::keys::v1::Address>,
    /// The incentivized asset.
    #[prost(message, optional, tag = "5")]
    pub incentivized_asset_id: ::core::option::Option<
        super::super::super::asset::v1::AssetId,
    >,
}
impl ::prost::Name for EventLqtDelegatorReward {
    const NAME: &'static str = "EventLqtDelegatorReward";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.EventLqtDelegatorReward".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.EventLqtDelegatorReward".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventLqtPositionReward {
    /// The epoch for which the reward was paid.
    #[prost(uint64, tag = "1")]
    pub epoch_index: u64,
    /// The amount of the reward, in staking tokens.
    #[prost(message, optional, tag = "2")]
    pub reward_amount: ::core::option::Option<super::super::super::num::v1::Amount>,
    /// The liquidity position receiving the reward
    #[prost(message, optional, tag = "3")]
    pub position_id: ::core::option::Option<super::super::dex::v1::PositionId>,
    /// The incentivized asset.
    #[prost(message, optional, tag = "4")]
    pub incentivized_asset_id: ::core::option::Option<
        super::super::super::asset::v1::AssetId,
    >,
    /// The total volume for the pair during the tournament, in staking tokens.
    #[prost(message, optional, tag = "10")]
    pub tournament_volume: ::core::option::Option<super::super::super::num::v1::Amount>,
    /// The cumulative volume for the LP, in staking tokens.
    #[prost(message, optional, tag = "11")]
    pub position_volume: ::core::option::Option<super::super::super::num::v1::Amount>,
}
impl ::prost::Name for EventLqtPositionReward {
    const NAME: &'static str = "EventLqtPositionReward";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.EventLqtPositionReward".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.EventLqtPositionReward".into()
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventLqtVote {
    /// The tournament for which the vote was cast.
    #[prost(uint64, tag = "1")]
    pub epoch_index: u64,
    /// The amount of voting power this vote carries.
    #[prost(message, optional, tag = "2")]
    pub voting_power: ::core::option::Option<super::super::super::num::v1::Amount>,
    /// The asset id of the asset being voted on.
    #[prost(message, optional, tag = "3")]
    pub incentivized_asset_id: ::core::option::Option<
        super::super::super::asset::v1::AssetId,
    >,
    /// The denom string of the asset being voted on.
    #[prost(message, optional, tag = "4")]
    pub incentivized: ::core::option::Option<super::super::super::asset::v1::Denom>,
    /// The beneficiary of the rewards this vote might receive.
    #[prost(message, optional, tag = "5")]
    pub rewards_recipient: ::core::option::Option<
        super::super::super::keys::v1::Address,
    >,
    /// The transaction ID of the vote.
    #[prost(message, optional, tag = "6")]
    pub tx_id: ::core::option::Option<super::super::super::txhash::v1::TransactionId>,
}
impl ::prost::Name for EventLqtVote {
    const NAME: &'static str = "EventLqtVote";
    const PACKAGE: &'static str = "penumbra.core.component.funding.v1";
    fn full_name() -> ::prost::alloc::string::String {
        "penumbra.core.component.funding.v1.EventLqtVote".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/penumbra.core.component.funding.v1.EventLqtVote".into()
    }
}
/// Generated client implementations.
#[cfg(feature = "rpc")]
pub mod funding_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Query operations for the funding component.
    #[derive(Debug, Clone)]
    pub struct FundingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl FundingServiceClient<tonic::transport::Channel> {
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
    impl<T> FundingServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
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
        ) -> FundingServiceClient<InterceptedService<T, F>>
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
            >>::Error: Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            FundingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Checks if a particular nullifier has already been used in the current epoch.
        pub async fn lqt_check_nullifier(
            &mut self,
            request: impl tonic::IntoRequest<super::LqtCheckNullifierRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LqtCheckNullifierResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::unknown(
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/penumbra.core.component.funding.v1.FundingService/LqtCheckNullifier",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "penumbra.core.component.funding.v1.FundingService",
                        "LqtCheckNullifier",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "rpc")]
pub mod funding_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value,
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with FundingServiceServer.
    #[async_trait]
    pub trait FundingService: std::marker::Send + std::marker::Sync + 'static {
        /// Checks if a particular nullifier has already been used in the current epoch.
        async fn lqt_check_nullifier(
            &self,
            request: tonic::Request<super::LqtCheckNullifierRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LqtCheckNullifierResponse>,
            tonic::Status,
        >;
    }
    /// Query operations for the funding component.
    #[derive(Debug)]
    pub struct FundingServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> FundingServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for FundingServiceServer<T>
    where
        T: FundingService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
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
            match req.uri().path() {
                "/penumbra.core.component.funding.v1.FundingService/LqtCheckNullifier" => {
                    #[allow(non_camel_case_types)]
                    struct LqtCheckNullifierSvc<T: FundingService>(pub Arc<T>);
                    impl<
                        T: FundingService,
                    > tonic::server::UnaryService<super::LqtCheckNullifierRequest>
                    for LqtCheckNullifierSvc<T> {
                        type Response = super::LqtCheckNullifierResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LqtCheckNullifierRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as FundingService>::lqt_check_nullifier(&inner, request)
                                    .await
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
                        let method = LqtCheckNullifierSvc(inner);
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
                        let mut response = http::Response::new(empty_body());
                        let headers = response.headers_mut();
                        headers
                            .insert(
                                tonic::Status::GRPC_STATUS,
                                (tonic::Code::Unimplemented as i32).into(),
                            );
                        headers
                            .insert(
                                http::header::CONTENT_TYPE,
                                tonic::metadata::GRPC_CONTENT_TYPE,
                            );
                        Ok(response)
                    })
                }
            }
        }
    }
    impl<T> Clone for FundingServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "penumbra.core.component.funding.v1.FundingService";
    impl<T> tonic::server::NamedService for FundingServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
