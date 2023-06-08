use crate::infra::{
    api::{Error, State},
    observe,
};

mod dto;

pub use dto::OrderError;
use tap::TapFallible;

pub(in crate::infra::api) fn quote(router: axum::Router<State>) -> axum::Router<State> {
    router.route("/quote", axum::routing::post(route))
}

async fn route(
    state: axum::extract::State<State>,
    order: axum::Json<dto::Order>,
) -> Result<axum::Json<dto::Quote>, (hyper::StatusCode, axum::Json<Error>)> {
    let order = order.0.into_domain().tap_err(|err| {
        observe::invalid_dto(err, "/quote", "order");
    })?;
    observe::quoting(&order);
    let quote = order
        .quote(state.eth(), state.solver(), state.liquidity(), state.now())
        .await;
    observe::quoted(&order, &quote);
    Ok(axum::response::Json(dto::Quote::new(&quote?)))
}
