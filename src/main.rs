use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_bedrockruntime::primitives::Blob;
use aws_sdk_bedrockruntime::{Client, Error}; // Binary format
use serde_json::Value;

fn parse_bedrock_to_json(s: &str) -> serde_json::Result<()> {
    let v: Value = serde_json::from_str(s)?;
    let f = format!("{}", v["results"][0]["outputText"]);
    println!("{}", f);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    // Call bedrock with client
    // Is & a pointer. Client gets to borrow config
    let bedrock_client = Client::new(&config);
    let prompt = "Describe que es la inteligencia artificial generativa";

    let request_body: Blob = Blob::new(format!(
        r#"{{
                 "inputText": "{}",
                 "textGenerationConfig": {{
                     "temperature": 0.0,
                     "topP": 1,
                     "maxTokenCount": 4096,
                     "stopSequences": []
                 }}     
            }}
            "#,
        prompt
    ));

    let response = bedrock_client
        .invoke_model()
        .body(request_body)
        .content_type("application/json")
        .accept("*/*")
        .model_id("amazon.titan-text-lite-v1")
        .send()
        .await;

    // println!("{:?}", response);
    let words: Vec<u8> = response.unwrap().body.into_inner();
    let words_utf8 = String::from_utf8(words);
    // println!("{}", words_utf8.unwrap());

    let _ = parse_bedrock_to_json(&words_utf8.unwrap());

    Ok(())
}
