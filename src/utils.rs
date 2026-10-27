// use rust_socketio::{asynchronous::Client, Payload};

use crate::{async_client::client::HttpClient, enums::Environment, ws_client::WsClient};

pub async fn create_client(
    env: Environment,
    private_key: &str,
    owner_address: Option<String>,
) -> anyhow::Result<(HttpClient, WsClient)> {
    let http_client = HttpClient::new(env, private_key, owner_address).await;
    let ws_client = WsClient::new(env);
    Ok((http_client, ws_client))
}

use crate::models::OrderSide;
use anyhow::{anyhow, Result};
use rust_decimal::{Decimal, RoundingStrategy};

/// Snaps a limit price onto the venue's tick without crossing in the taker direction: a buy
/// rounds down, a sell rounds up. Rounding to the nearest tick could move a buy up through the
/// spread (which a post-only order is then rejected for) and used banker's rounding, so two
/// adjacent half-tick prices rounded in opposite directions. A zero tick leaves the price as is.
pub fn round_price_to_tick(price: Decimal, tick_size: Decimal, side: OrderSide) -> Result<Decimal> {
    if price.is_sign_negative() || price.is_zero() {
        return Err(anyhow!("a limit price must be positive, got {}", price));
    }
    if tick_size.is_zero() {
        return Ok(price);
    }
    let strategy = match side {
        OrderSide::BUY => RoundingStrategy::ToNegativeInfinity,
        OrderSide::SELL => RoundingStrategy::ToPositiveInfinity,
    };
    let ticks = (price / tick_size).round_dp_with_strategy(0, strategy);
    Ok(ticks * tick_size)
}

/// Floors a quantity onto the venue's lot size and refuses one that floors to nothing. Rounding
/// to the nearest lot took a quantity at or below half a lot to zero, and a zero-quantity order
/// was then signed and submitted; rounding up would send more than the caller sized.
pub fn floor_quantity_to_lot(quantity: Decimal, lot_size: Decimal) -> Result<Decimal> {
    if quantity.is_sign_negative() || quantity.is_zero() {
        return Err(anyhow!(
            "an order quantity must be positive, got {}",
            quantity
        ));
    }
    if lot_size.is_zero() {
        return Ok(quantity);
    }
    let lots =
        (quantity / lot_size).round_dp_with_strategy(0, RoundingStrategy::ToNegativeInfinity);
    let floored = lots * lot_size;
    if floored.is_zero() {
        return Err(anyhow!(
            "quantity {} is below one lot of {}; refusing to send a zero-quantity order",
            quantity,
            lot_size
        ));
    }
    Ok(floored)
}
