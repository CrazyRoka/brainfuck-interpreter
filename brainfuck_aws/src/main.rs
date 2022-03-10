use brainfuck_interpreter::interpret;
use lambda_http::{service_fn, Error, IntoResponse, Request, RequestExt, Response};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct InterpreterRequest {
    source: String,
    input: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
enum InterpreterResponse {
    Success(String),
    Error(String),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    info!("Starting lambda function!");

    let func = service_fn(func);
    lambda_http::run(func).await?;
    Ok(())
}

async fn func(event: Request) -> Result<impl IntoResponse, Error> {
    debug!("Received request: {:?}", event);
    info!("Processing request!");

    match process_request(event).await {
        Ok(result) => Ok(Response::builder()
            .status(200)
            .body(serde_json::to_string(&result)?)?),
        Err(error) => Ok(Response::builder().status(400).body(serde_json::to_string(
            &InterpreterResponse::Error(error.to_string()),
        )?)?),
    }
}

async fn process_request(request: Request) -> Result<InterpreterResponse, Error> {
    if let Some(request) = request.payload::<InterpreterRequest>()? {
        debug!("Body is valid. Processing request");
        let source = request.source;
        let input = request.input.unwrap_or(String::new());
        let stdin = Box::new(input.as_bytes());

        let result = match interpret(&source, stdin) {
            Ok(output) => InterpreterResponse::Success(output),
            Err(error) => InterpreterResponse::Error(error.to_string()),
        };
        info!("Interpreter result: {:?}", result);

        Ok(result)
    } else {
        warn!("Can't process request. Invalid body");
        Err("Invalid body")?
    }
}
