// src/generator/mod.rs

use crate::core::model::Api;
use crate::error::Result;

/// A trait for things that can generate output from an Api model.
pub trait Generator {
    /// Generates a string output from the given Api model.
    fn generate(&self, api: &Api) -> Result<String>;
}

/// A generator that produces Markdown output.
pub struct MarkdownGenerator;

impl Generator for MarkdownGenerator {
    fn generate(&self, api: &Api) -> Result<String> {
        let mut output = String::new();
        output.push_str(&format!("# {}\n", api.title));
        output.push_str(&format!("Version: {}\n\n", api.version));

        if !api.endpoints.is_empty() {
            output.push_str("## Endpoints\n\n");
            for endpoint in &api.endpoints {
                output.push_str(&format!(
                    "- `{:?}` **{}**: {}\n",
                    endpoint.method,
                    endpoint.path,
                    endpoint.summary
                ));
            }
        }

        Ok(output)
    }
}

/// A generator that produces a Markdown development plan.
pub struct PlanGenerator;

impl Generator for PlanGenerator {
    fn generate(&self, api: &Api) -> Result<String> {
        let mut output = String::new();
        output.push_str("# Development Plan\n\n");
        output.push_str("Based on the API specification, the following tasks are proposed:\n\n");

        if api.endpoints.is_empty() {
            output.push_str("- No endpoints defined. Nothing to plan.");
        } else {
            for endpoint in &api.endpoints {
                output.push_str(&format!(
                    "- [ ] Implement endpoint: `{:?} {}` (Summary: {})\n",
                    endpoint.method, endpoint.path, endpoint.summary
                ));
            }
        }

        Ok(output)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::model::{Api, Endpoint, HttpMethod};

    #[test]
    fn markdown_generator_produces_correct_output() {
        let api = Api {
            title: "My Test API".to_string(),
            version: "v1.0".to_string(),
            endpoints: vec![
                Endpoint {
                    path: "/users".to_string(),
                    method: HttpMethod::Get,
                    summary: "Get users".to_string(),
                },
                Endpoint {
                    path: "/users".to_string(),
                    method: HttpMethod::Post,
                    summary: "Create user".to_string(),
                },
            ],
        };

        let generator = MarkdownGenerator;
        let output = generator.generate(&api).unwrap();

        let expected_output = "# My Test API\nVersion: v1.0\n\n## Endpoints\n\n- `Get` **/users**: Get users\n- `Post` **/users**: Create user\n";
        assert_eq!(output, expected_output);
    }
}