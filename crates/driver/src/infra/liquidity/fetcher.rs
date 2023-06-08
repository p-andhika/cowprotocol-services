use {
    crate::{
        boundary,
        domain::liquidity,
        infra::{self, blockchain::Ethereum, observe},
    },
    std::{collections::HashSet, sync::Arc},
};

/// Fetch liquidity for auctions to be sent to solver engines.
#[derive(Clone, Debug)]
pub struct Fetcher {
    inner: Arc<boundary::liquidity::Fetcher>,
}

impl Fetcher {
    /// Creates a new liquidity fetcher for the specified Ethereum instance and
    /// configuration.
    pub async fn new(eth: &Ethereum, config: &infra::liquidity::Config) -> Result<Self, Error> {
        let inner = boundary::liquidity::Fetcher::new(eth, config).await?;
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    /// Fetches all relevant liquidity for the specified token pairs. Handles
    /// failures by logging and returning an empty vector.
    pub async fn fetch(&self, pairs: &HashSet<liquidity::TokenPair>) -> Vec<liquidity::Liquidity> {
        observe::fetching_liquidity();
        match self.inner.fetch(pairs).await {
            Ok(liquidity) => {
                observe::fetched_liquidity(&liquidity);
                liquidity
            }
            Err(e) => {
                observe::fetching_liquidity_failed(&e);
                Default::default()
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("boundary error: {0:?}")]
    Boundary(#[from] boundary::Error),
}
