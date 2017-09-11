// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use cache_2q::Cache;
use libproto::auth::*;
use util::H256;

#[derive(Debug)]
pub struct VerifyCache {
    inner: Cache<H256, VerifyTxResp>,
}

impl VerifyCache {
    pub fn new(size: usize) -> Self {
        // size x4  because cache_2q
        VerifyCache { inner: Cache::new(size * 4) }
    }

    pub fn insert(&mut self, tx_hash: H256, resp: VerifyTxResp) {
        self.inner.insert(tx_hash, resp);
    }

    pub fn get(&self, tx_hash: &H256) -> Option<&VerifyTxResp> {
        self.inner.peek(tx_hash)
    }
}


#[test]
fn basic() {
    let mut cache = VerifyCache::new(2);

    let hash1 = H256::from_slice(&vec![1]);
    let hash2 = H256::from_slice(&vec![2]);
    let hash3 = H256::from_slice(&vec![3]);

    let mut resp1 = VerifyTxResp::new();
    resp1.set_tx_hash(hash1.to_vec());
    let mut resp2 = VerifyTxResp::new();
    resp2.set_tx_hash(hash2.to_vec());
    let mut resp3 = VerifyTxResp::new();
    resp3.set_tx_hash(hash3.to_vec());

    cache.insert(hash1.clone(), resp1.clone());
    cache.insert(hash2.clone(), resp2.clone());
    cache.insert(hash3.clone(), resp3.clone());

    assert_eq!(cache.get(&hash1), None);
    assert_eq!(cache.get(&hash2), Some(&resp2));
    assert_eq!(cache.get(&hash3), Some(&resp3));
}
