//
//  Quality Cluster HRI04 - Strive for codestyle consistency
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

const DOT_CLANG_FORMAT: &'static str = ".clang-format";
const RUSTFMT_TOML: &'static str = "rustfmt.toml";
const DOT_RUSTFMT_TOML: &'static str = ".rustfmt.toml";
const PYPROJECT_TOML: &'static str = "pyproject.toml";

pub fn run() -> CheckStatus {
    info!("checking HRI04 (Strive for codestyle-consistency)");

    let mut results: [CheckStatus; 3] = [CheckStatus::NotApplicable; 3];

    results[0] = have_clangformat();
    results[1] = have_rustfmt();
    results[2] = have_ruff();

    let mut incomplete = false;

    for i in 0..results.len() {
        match results[i] {
            CheckStatus::Success => {
                info!("HRI04 passed ✅");
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
            warn!("HRI04 incomplete ⏳");
            CheckStatus::Incomplete
        }
        false => {
            error!("no formatter config found");
            error!("HRI04 failed ❌");
            CheckStatus::Failure
        }
    }
}

fn have_clangformat() -> CheckStatus {
    if file_exists(DOT_CLANG_FORMAT) {
        info!("found {}", DOT_CLANG_FORMAT);
        CheckStatus::Success
    } else {
        CheckStatus::Failure
    }
}

fn have_ruff() -> CheckStatus {
    match fs::read_to_string(PYPROJECT_TOML) {
        Ok(content) => {
            info!("found {}", PYPROJECT_TOML);

            if content.contains("[tool.ruff.format]") {
                info!("found ruff formatter config");
                CheckStatus::Success
            } else {
                warn!("ruff formatter config not found");
                CheckStatus::Incomplete
            }
        }
        Err(_) => CheckStatus::Failure,
    }
}

fn have_rustfmt() -> CheckStatus {
    if file_exists(RUSTFMT_TOML) {
        info!("found {}", RUSTFMT_TOML);
        CheckStatus::Success
    } else if file_exists(DOT_RUSTFMT_TOML) {
        info!("found {}", RUSTFMT_TOML);
        CheckStatus::Success
    } else {
        CheckStatus::Failure
    }
}

fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
