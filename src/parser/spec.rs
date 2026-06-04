// src/parser/spec.rs
use serde::Deserialize;
use std::collections::HashMap;

/// The root document of the OpenAPI document.
#[derive(Debug, Deserialize, PartialEq)]
pub struct OpenApiSpec {
    /// The OpenAPI specification version.
    pub openapi: String,
    /// Provides metadata about the API.
    pub info: Info,
    /// The available paths and operations for the API.
    #[serde(default)]
    pub paths: HashMap<String, PathItem>,
}

/// The object provides metadata about the API.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Info {
    /// The title of the API.
    pub title: String,
    /// The version of the API.
    pub version: String,
}

/// Describes the operations available on a single path.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    // Add other HTTP methods (put, delete, etc.) here as needed
}

/// Describes a single API operation on a path.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    /// A short summary of what the operation does.
    #[serde(default)]
    pub summary: String,
    /// A unique string to identify the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
}