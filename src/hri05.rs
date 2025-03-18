//
//  Quality Cluster HRI05 - Use static source-code analyzers
//
//  Copyright (c) 2024-2025, Honda Research Institute Europe GmbH
//
//  Redistribution and use in source and binary forms, with or without
//  modification, are permitted provided that the following conditions are
//  met:
//
//  1. Redistributions of source code must retain the above copyright notice,
//     this list of conditions and the following disclaimer.
//
//  2. Redistributions in binary form must reproduce the above copyright
//     notice, this list of conditions and the following disclaimer in the
//     documentation and/or other materials provided with the distribution.
//
//  3. Neither the name of the copyright holder nor the names of its
//     contributors may be used to endorse or promote products derived from
//     this software without specific prior written permission.
//
//  THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
//  IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
//  THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
//  PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
//  CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
//  EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
//  PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
//  PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
//  LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
//  NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
//  SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
//  SPDX-License-Identifier: BSD-3-Clause
//

use crate::CheckStatus;
use log::{error, info, warn};
use std::fs;

const CLIPPY_TOML: &str = "clippy.toml";

const DOT_CLANG_TIDY: &str = ".clang-tidy";
const PYPROJECT_TOML: &str = "pyproject.toml";

pub fn run() -> CheckStatus {
    info!("checking HRI05 (Use static source-code analyzers)");
    let mut results: [CheckStatus; 3] = [CheckStatus::NotApplicable; 3];

    results[0] = have_clangtidy();
    results[1] = have_clippy();
    results[2] = have_ruff();

    let mut incomplete = false;

    for item in &results {
        match item {
            CheckStatus::Success => {
                info!("HRI05 passed ✅");
                return CheckStatus::Success;
            }
            CheckStatus::Incomplete => {
                incomplete = true;
            }
            _ => {}
        }
    }

    match incomplete {
        true => {
            warn!("HRI05 incomplete ⏳");
            CheckStatus::Incomplete
        }
        false => {
            error!("no linter config found");
            error!("HRI05 failed ❌");
            CheckStatus::Failure
        }
    }
}

fn have_clangtidy() -> CheckStatus {
    if file_exists(DOT_CLANG_TIDY) {
        info!("found {}", DOT_CLANG_TIDY);
        CheckStatus::Success
    } else {
        CheckStatus::Failure
    }
}

fn have_clippy() -> CheckStatus {
    if file_exists(CLIPPY_TOML) {
        info!("found {}", CLIPPY_TOML);
        CheckStatus::Success
    } else {
        CheckStatus::Failure
    }
}

fn have_ruff() -> CheckStatus {
    match fs::read_to_string(PYPROJECT_TOML) {
        Ok(content) => {
            info!("found {}", PYPROJECT_TOML);

            if content.contains("[tool.ruff.lint") {
                info!("found ruff linter config");
                CheckStatus::Success
            } else {
                warn!("ruff linter config not found");
                CheckStatus::Incomplete
            }
        }
        Err(_) => CheckStatus::Failure,
    }
}

fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
