use serde;
use utoipa::ToSchema;
#[derive(
    Copy,
    Default,
    Clone,
    Debug,
    Eq,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    strum::Display,
    strum::EnumString,
    ToSchema,
    Hash,
)]
#[router_derive::diesel_enum(storage_type = "text")]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MerchantProductType {
    #[default]
    Orchestration,
    Vault,
    Recon,
    Recovery,
    CostObservability,
    DynamicRouting,
}
