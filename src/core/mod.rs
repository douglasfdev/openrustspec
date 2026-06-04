// src/core/mod.rs

pub mod model;

use crate::error::Result;
use crate::parser::spec::OpenApiSpec;
use model::{Api, Endpoint, HttpMethod};

/// Processes the parsed specification into a domain model.
pub fn process(spec: OpenApiSpec) -> Result<Api> {
    let mut endpoints = Vec::new();

    for (path, path_item) in spec.paths {
        if let Some(op) = path_item.get {
            endpoints.push(Endpoint {
                path: path.clone(),
                method: HttpMethod::Get,
                summary: op.summary,
            });
        }
        if let Some(op) = path_item.post {
            endpoints.push(Endpoint {
                path: path.clone(),
                method: HttpMethod::Post,
                summary: op.summary,
            });
        }
    }

    Ok(Api {
        title: spec.info.title,
        version: spec.info.version,
        endpoints,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::spec::{Info, Operation, PathItem};
    use std::collections::HashMap;

    #[test]
    fn processes_spec_with_paths_into_api_model() {
        let mut paths = HashMap::new();
        paths.insert(
            "/users".to_string(),
            PathItem {
                get: Some(Operation {
                    summary: "Get all users".to_string(),
                    operation_id: None,
                }),
                post: None,
            },
        );
        paths.insert(
            "/users/{id}".to_string(),
            PathItem {
                get: Some(Operation {
                    summary: "Get a user by ID".to_string(),
                    operation_id: None,
                }),
                post: None,
            },
        );

        let spec = OpenApiSpec {
            openapi: "3.0.0".to_string(),
            info: Info {
                title: "Test API".to_string(),
                version: "1.2.3".to_string(),
            },
            paths,
        };

        let api = process(spec).unwrap();

        assert_eq!(api.title, "Test API");
        assert_eq!(api.endpoints.len(), 2);

        // Note: HashMap iteration order is not guaranteed, so we find the endpoints
        let get_users = api.endpoints.iter().find(|e| e.path == "/users").unwrap();
        assert_eq!(get_users.method, HttpMethod::Get);
        assert_eq!(get_users.summary, "Get all users");

        let get_user_by_id = api
            .endpoints
            .iter()
            .find(|e| e.path == "/users/{id}")
            .unwrap();
        assert_eq!(get_user_by_id.method, HttpMethod::Get);
        assert_eq!(get_user_by_id.summary, "Get a user by ID");
    }
}