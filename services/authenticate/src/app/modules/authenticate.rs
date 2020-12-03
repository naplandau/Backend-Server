use crate::app::lib::*;
pub async fn check_token1(token: Token) -> Result<TokenResult, ServerError> {
    let now = Instant::now();
    match token.validate() {
        Ok(_) => {
            unimplemented!();
        }
        Err(e) => {
            println!(
                "[MODULES] | [CHECK_TOKEN] | [{:?}]",
                now.elapsed().as_secs_f32()
            );
            Err(ServerError::from(e))
        }
    }
}