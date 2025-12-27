mod serde_ex;
pub use pipewire;

pub mod api;

pub mod apo;
pub mod config;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::process::Command;

use self::serde_ex::KeyValuePairs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwDumpObject {
    pub id: u32,
    #[serde(rename = "type")]
    pub object_type: PwObjectType,
    #[serde(default)]
    pub version: Option<u32>,
    #[serde(default)]
    pub permissions: Option<Vec<String>>,
    #[serde(default)]
    pub info: PwObjectInfo,
    #[serde(default)]
    pub props: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PwObjectType {
    #[serde(rename = "PipeWire:Interface:Core")]
    Core,
    #[serde(rename = "PipeWire:Interface:Module")]
    Module,
    #[serde(rename = "PipeWire:Interface:Client")]
    Client,
    #[serde(rename = "PipeWire:Interface:SecurityContext")]
    SecurityContext,
    #[serde(rename = "PipeWire:Interface:Profiler")]
    Profiler,
    #[serde(rename = "PipeWire:Interface:Factory")]
    Factory,
    #[serde(rename = "PipeWire:Interface:Device")]
    Device,
    #[serde(rename = "PipeWire:Interface:Metadata")]
    Metadata,
    #[serde(rename = "PipeWire:Interface:Node")]
    Node,
    #[serde(rename = "PipeWire:Interface:Port")]
    Port,
    #[serde(rename = "PipeWire:Interface:Link")]
    Link,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PwObjectInfo {
    #[serde(default)]
    pub props: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub params: PwParams,
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PwParams {
    #[serde(default)]
    pub enum_format: Vec<serde_json::Value>,
    #[serde(default)]
    pub prop_info: Vec<PwPropInfo>,
    #[serde(default)]
    pub props: Vec<Prop>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Prop {
    #[serde(default)]
    pub params: KeyValuePairs<HashMap<String, serde_json::Value>>,
    #[serde(flatten)]
    pub fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwPropInfo {
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "type")]
    pub type_: serde_json::Value,
}

pub async fn dump() -> Result<Vec<PwDumpObject>> {
    let output = Command::new("pw-dump")
        .output()
        .await
        .context("failed to execute pw-dump")?;

    if !output.status.success() {
        anyhow::bail!(
            "pw-dump failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let json_str = String::from_utf8(output.stdout).context("pw-dump output is not valid UTF-8")?;

    let objects: Vec<PwDumpObject> =
        serde_json::from_str(&json_str).context("Failed to parse pw-dump JSON")?;

    Ok(objects)
}

pub async fn set_default(node_id: u32) -> Result<()> {
    let output = Command::new("wpctl")
        .arg("set-default")
        .arg(node_id.to_string())
        .output()
        .await
        .context("Failed to execute wpctl")?;

    if !output.status.success() {
        anyhow::bail!("wpctl failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}

pub fn to_spa_json<T: serde::Serialize>(value: &T) -> String {
    let json_value = serde_json::to_value(value).expect("Failed to serialize to JSON value");
    self::config::SpaJson::new(&json_value).to_string()
}
