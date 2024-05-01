use async_trait::async_trait;

use super::OutputParserError;

#[async_trait]
pub trait OutputParser<O>: Send + Sync {
    async fn parse(&self, output: &str) -> Result<O, OutputParserError>;
}
