#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestDiagnosticSettingsAssociationList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GuestDiagnosticSettingsAssociationResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestDiagnosticSettingsAssociationResourcePatch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GuestDiagnosticSettingsAssociation>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestDiagnosticSettingsAssociation {
    #[serde(rename = "guestDiagnosticSettingsName")]
    pub guest_diagnostic_settings_name: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestDiagnosticSettingsAssociationResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: GuestDiagnosticSettingsAssociation,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
    pub location: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestDiagnosticSettingsList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<GuestDiagnosticSettingsResource>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestDiagnosticSettingsResource {
    #[serde(flatten)]
    pub resource: Resource,
    pub properties: GuestDiagnosticSettings,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestDiagnosticSettings {
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<guest_diagnostic_settings::OsType>,
    #[serde(rename = "dataSources", skip_serializing_if = "Vec::is_empty")]
    pub data_sources: Vec<DataSource>,
    #[serde(rename = "proxySetting", skip_serializing_if = "Option::is_none")]
    pub proxy_setting: Option<String>,
}
pub mod guest_diagnostic_settings {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GuestDiagnosticSettingsPatchResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<GuestDiagnosticSettings>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSource {
    pub kind: data_source::Kind,
    pub configuration: DataSourceConfiguration,
    pub sinks: Vec<SinkConfiguration>,
}
pub mod data_source {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        PerformanceCounter,
        #[serde(rename = "ETWProviders")]
        EtwProviders,
        WindowsEventLogs,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SinkConfiguration {
    pub kind: sink_configuration::Kind,
}
pub mod sink_configuration {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Kind {
        EventHub,
        ApplicationInsights,
        LogAnalytics,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataSourceConfiguration {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub providers: Vec<EtwProviderConfiguration>,
    #[serde(rename = "perfCounters", skip_serializing_if = "Vec::is_empty")]
    pub perf_counters: Vec<PerformanceCounterConfiguration>,
    #[serde(rename = "eventLogs", skip_serializing_if = "Vec::is_empty")]
    pub event_logs: Vec<EventLogConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EtwEventConfiguration {
    pub name: String,
    pub id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EtwProviderConfiguration {
    pub id: String,
    pub events: Vec<EtwEventConfiguration>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceCounterConfiguration {
    pub name: String,
    #[serde(rename = "samplingPeriod")]
    pub sampling_period: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventLogConfiguration {
    #[serde(rename = "logName")]
    pub log_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
}