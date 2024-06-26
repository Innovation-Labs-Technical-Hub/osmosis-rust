use osmosis_std_derive::CosmwasmExt;
/// ClientState defines a loopback (localhost) client. It requires (read-only)
/// access to keys outside the client prefix.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/ibc.lightclients.localhost.v1.ClientState")]
pub struct ClientState {
    /// self chain ID
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// self latest block height
    #[prost(message, optional, tag = "2")]
    pub height: ::core::option::Option<super::super::super::core::client::v1::Height>,
}
