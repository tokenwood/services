//! DTOs modeling the HTTP REST interface of the solver.

mod auction;
mod notification;
mod solution;

pub use {
    auction::Auction, auction::Liquidity, auction::WeightedProductVersion,
    notification::Notification, solution::Solutions,
};

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct Error(pub String);
