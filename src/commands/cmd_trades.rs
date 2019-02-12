use crate::error::*;
use crate::utils::*;
use cryptotrader;
use cryptotrader::{exchanges::*, models::*};
use log::info;

pub fn fetch<E>(
    client: E,
    symbol: &str,
    limit: Option<usize>,
    group: bool,
) -> CliResult<Vec<Vec<Trade>>>
where
    E: ExchangeAPI,
{
    let pairs = client.all_pairs()?;
    let pairs = find_all_pairs_by_symbol(&symbol, pairs.clone());

    info!(
        "found pairs: {}",
        string_map(pairs.clone(), { |p| p.base }).join(" ")
    );

    let mut all_trades: Vec<Vec<Trade>> = Vec::new();

    for pair in pairs {
        let trades = client.trades_for_pair(pair)?;
        let trades = optional_limit(limit, trades);

        let trades = match group {
            true => average_trades(trades),
            false => trades,
        };

        all_trades.push(trades);
    }

    Ok(all_trades)
}
