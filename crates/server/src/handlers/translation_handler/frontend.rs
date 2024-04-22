use crate::handlers::translation_handler::{Actions, ApiResponse, HandlerData, NumberOfKeysResponse};

pub struct FrontendServerHandler;

impl FrontendServerHandler {
    pub async fn match_action(actions: Actions, data: Option<HandlerData>) -> Result<ApiResponse, rspc::Error> {
        Ok(ApiResponse::NumberOfKeysResponse(NumberOfKeysResponse {
            num_of_keys: 0,
        }))
    }
}
