
use async_trait::async_trait;
use openspec_core::{
    domain::ports::llm_provider::{self, GenerateSpecRequest, GenerateSpecResponse, LlmProvider},
    error::{Error, Result},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Concrete adapter for the Ollama LLM.
pub struct OllamaAdapter {
    client: Client,
    model: String,
    endpoint: String,
}

impl OllamaAdapter {
    pub fn new(model: String, endpoint: String) -> Result<Self> {
        let client = Client::builder().build().map_err(|e| Error::Infrastructure(e.to_string()))?;
        Ok(Self {
            client,
            model,
            endpoint,
        })
    }
}

#[async_trait]
impl LlmProvider for OllamaAdapter {
    async fn generate_spec(
        &self,
        request: GenerateSpecRequest,
    ) -> Result<GenerateSpecResponse> {
        let prompt = format!(
            "Based on the following objective and context, generate a YAML specification for the task.\n\nObjective: {}\n\nContext: {}\n\nYAML Spec:"
            , request.objective, request.context
        );

        let ollama_request = OllamaGenerateRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
        };

        let response = self.client.post(&self.endpoint).json(&ollama_request).send().await.map_err(|e| Error::LlmProvider(e.to_string()))?;

        if !response.status().is_success() {
            return Err(Error::LlmProvider(format!(
                "Ollama API returned non-success status: {}",
                response.status()
            )));
        }

        let ollama_response: OllamaGenerateResponse = response.json().await.map_err(|e| Error::LlmProvider(e.to_string()))?;

        Ok(GenerateSpecResponse {
            spec_yaml: ollama_response.response,
        })
    }
}

// --- Ollama specific DTOs ---

#[derive(Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}