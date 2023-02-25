// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo library.

// The Aleo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo library. If not, see <https://www.gnu.org/licenses/>.

use super::ProgramManager;
use snarkvm_console::program::Network;

use crate::program::Resolver;
use anyhow::{anyhow, bail, Result};

use crate::Encryptor;
use snarkvm_console::account::PrivateKey;

impl<N: Network, R: Resolver<N>> ProgramManager<N, R> {
    /// Get a checked private key
    pub(super) fn get_private_key(&self, password: Option<String>) -> Result<PrivateKey<N>> {
        if self.private_key.is_none() && self.private_key_ciphertext.is_none() {
            bail!("Private key is not configured");
        };
        if let Some(private_key) = &self.private_key {
            if self.private_key_ciphertext.is_some() {
                bail!(
                    "Private key ciphertext is also configured, cannot have both private key and private key ciphertext"
                );
            }
            return Ok(*private_key);
        };
        if let Some(ciphertext) = &self.private_key_ciphertext {
            if self.private_key.is_some() {
                bail!("Private key is already configured, annot have both private key and private key ciphertext");
            }

            let password = password.ok_or_else(|| anyhow!("Private key is encrypted, password is required"))?;
            return Encryptor::<N>::decrypt_private_key_with_secret(ciphertext, &password);
        };
        bail!("Private key configuration error")
    }
}
