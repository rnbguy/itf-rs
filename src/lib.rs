use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct StateMeta {
    index: usize,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct TraceMeta {
    format: String,
    format_description: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
enum Tagged {
    #[serde(rename = "#bigint")]
    BigInt(i64),
    #[serde(rename = "#map")]
    Function(Vec<(ITFValue, ITFValue)>),
    #[serde(rename = "#set")]
    Set(Vec<ITFValue>),
    #[serde(rename = "#seq")]
    Sequence(Vec<ITFValue>),
    #[serde(rename = "#tup")]
    Tuple(Vec<ITFValue>),
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
enum Untagged {
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<ITFValue>),
    Object(HashMap<String, ITFValue>),
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
#[serde(untagged)]
enum ITFValue {
    T(Tagged),
    UT(Option<Untagged>),
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct State {
    #[serde(rename = "#meta")]
    meta: Option<StateMeta>,
    #[serde(flatten)]
    vars: HashMap<String, ITFValue>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Trace {
    #[serde(rename = "#meta")]
    meta: Option<TraceMeta>,
    vars: Vec<String>,
    states: Vec<State>,
}
