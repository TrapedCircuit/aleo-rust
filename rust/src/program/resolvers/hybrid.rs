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

use super::Resolver;
use snarkvm_console::network::Network;
use snarkvm_synthesizer::Program;

use crate::NetworkConfig;
use anyhow::{bail, ensure, Result};

use snarkvm_console::program::{Ciphertext, Plaintext, ProgramID, Record};
use std::path::PathBuf;

/// Hybrid resolver that uses a combination of local file system and network imports
///
/// The default behavior is to first attempt to load a program from the local file system,
/// and if that fails, to attempt to load it from programs stored on the aleo network.
pub struct HybridResolver<N: Network> {
    network_config: NetworkConfig,
    local_config: PathBuf,
    _phantom: core::marker::PhantomData<N>,
}

impl<N: Network> HybridResolver<N> {
    /// Create a new hybrid resolver
    pub fn new(network_config: &NetworkConfig, local_config: &PathBuf) -> Result<Self> {
        ensure!(local_config.exists(), "Path does not exist");
        ensure!(local_config.is_dir(), "Path is not a directory");
        Ok(Self {
            network_config: network_config.clone(),
            local_config: local_config.clone(),
            _phantom: core::marker::PhantomData,
        })
    }

    pub fn import_directory(&mut self) -> PathBuf {
        let mut import_directory = self.local_config.clone();
        import_directory.push("/imports");
        import_directory
    }
}

impl<N: Network> Resolver<N> for HybridResolver<N> {
    const NAME: &'static str = "FileSystemResolver";

    fn load_program(&self, _program_id: &ProgramID<N>) -> Result<Program<N>> {
        bail!("A functional resolver is required to load programs, please configure one");
    }

    fn resolve_program_imports(&self, _program: &Program<N>) -> Result<Vec<(ProgramID<N>, Result<Program<N>>)>> {
        bail!("A functional resolver is required to resolve imports, please configure one");
    }

    fn find_records(&self) -> Result<Vec<Record<N, Ciphertext<N>>>> {
        bail!("A functional resolver is required to find records, please configure one");
    }

    fn find_unspent_records(&self) -> Result<Vec<Record<N, Plaintext<N>>>> {
        bail!("A functional resolver is required to find records, please configure one");
    }
}
