// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

use crate::jsonrpc_types::StrView;
use anyhow::Result;
use bitcoin::Txid;
use std::fmt;
use std::str::FromStr;

pub type TxidView = StrView<Txid>;

impl fmt::Display for TxidView {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for TxidView {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StrView(Txid::from_str(s)?))
    }
}

impl From<TxidView> for Txid {
    fn from(value: TxidView) -> Self {
        value.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_txid() -> Result<()> {
        let txid_str = "5fddcbdc3eb21a93e8dd1dd3f9087c3677f422b82d5ba39a6b1ec37338154af6";
        let txid_view = TxidView::from_str(txid_str)?;
        let txid = Txid::from_str(txid_str)?;
        let txid_str2 = txid.to_string();
        assert!(txid_view.0 == txid);
        assert!(txid_str == txid_str2);

        Ok(())
    }
}
