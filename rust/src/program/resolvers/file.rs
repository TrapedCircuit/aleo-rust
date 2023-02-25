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

use snarkvm_console::network::Network;
use snarkvm_synthesizer::Program;

use super::Resolver;

use anyhow::{ensure, Result};

use snarkvm_console::program::{Ciphertext, Plaintext, ProgramID, Record};

use std::path::PathBuf;

use snarkvm::{
    file::{AleoFile, Manifest},
    package::Package,
};

/// Resolver for imports from the local file system
pub struct FileSystemResolver<N: Network> {
    local_config: PathBuf,
    _phantom: core::marker::PhantomData<N>,
}

impl<N: Network> FileSystemResolver<N> {
    /// Create a new file system resolver
    pub fn new(local_config: &PathBuf) -> Result<Self> {
        ensure!(local_config.exists(), "Path does not exist");
        ensure!(local_config.is_dir(), "Path is not a directory");
        Ok(Self { local_config: local_config.clone(), _phantom: core::marker::PhantomData })
    }

    pub fn import_directory(&self) -> PathBuf {
        self.local_config.join("/imports")
    }
}

impl<N: Network> Resolver<N> for FileSystemResolver<N> {
    const NAME: &'static str = "FileSystemResolver";

    fn load_program(&self, program_id: &ProgramID<N>) -> Result<Program<N>> {
        // Ensure the directory path exists.
        ensure!(self.local_config.exists(), "The program directory does not exist");

        ensure!(
            Manifest::<N>::exists_at(&self.local_config),
            "Please ensure that the manifest file exists in the Aleo program directory (missing '{}' at '{}')",
            Manifest::<N>::file_name(),
            &self.local_config.display()
        );

        // Open the manifest file.
        let manifest = Manifest::<N>::open(&self.local_config)?;
        ensure!(
            manifest.program_id() == program_id,
            "The program name in the manifest file does not match the specified program name"
        );

        // Load the package.
        let package = Package::open(&self.local_config)?;
        // Load the main program.
        Ok(package.program().clone())
    }

    fn resolve_program_imports(&self, program: &Program<N>) -> Result<Vec<(ProgramID<N>, Result<Program<N>>)>> {
        program
            .imports()
            .keys()
            .map(|program_id| {
                // Open the Aleo program file.
                let program = AleoFile::open(&self.import_directory(), program_id, false)
                    .and_then(|aleo_file| Ok(aleo_file.program().clone()))
                    .map_err(|err| anyhow::anyhow!(err.to_string()));
                Ok((program_id.clone(), program))
            })
            .collect::<Result<Vec<(ProgramID<N>, Result<Program<N>>)>>>()
    }

    fn find_records(&self) -> Result<Vec<Record<N, Ciphertext<N>>>> {
        Ok(vec![])
    }

    fn find_unspent_records(&self) -> Result<Vec<Record<N, Plaintext<N>>>> {
        Ok(vec![])
    }
}
