// src/core/model.rs

//! Contains the internal domain models, which are agnostic of the input spec format.

/// Represents a processed API, ready for generation tasks.
#[derive(Debug, PartialEq)]
pub struct Api {
    /// The title of the API.
    pub title: String,
    /// The version of the API.
    pub version: String,
    /// The list of endpoints in the API.
    pub endpoints: Vec<Endpoint>,
}

/// Represents a single API endpoint.
#[derive(Debug, PartialEq)]
pub struct Endpoint {
    /// The path of the endpoint (e.g., "/users").
    pub path: String,
    /// The HTTP method for this endpoint.
    pub method: HttpMethod,
    /// A short summary of what the endpoint does.
    pub summary: String,
}

/// Represents an HTTP method.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Post,
    // Add other methods as needed
}