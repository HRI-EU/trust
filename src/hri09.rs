//
//  Quality Cluster HRI09 - Use a modern build / packaging system
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

const CONANFILE_PY: &'static str = "conanfile.py";
const CONANFILE_TXT: &'static str = "conanfile.txt";
const CONAN_LOCK: &'static str = "conan.lock";

const CARGO_TOML: &'static str = "Cargo.toml";
const CARGO_LOCK: &'static str = "Cargo.lock";

const CONDA_LOCK: &'static str = "conda.lock";

const ENVIRONMENT_YML: &'static str = "environment.yml";
const ENVIRONMENT_YAML: &'static str = "environment.yaml";
const PYPROJECT_TOML: &'static str = "pyproject.toml";
const UV_TOML: &'static str = "uv.toml";
const UV_LOCK: &'static str = "uv.lock";

const PACKAGE_JSON: &'static str = "package.json";
const PACKAGE_LOCK_JSON: &'static str = "package-lock.json";



pub fn run() -> CheckStatus {
    info!("checking HRI09 (Use modern build/packaging system)");

    let mut results: [CheckStatus; 5] = [CheckStatus::NotApplicable; 5];

    results[0] = have_cargo();
    results[1] = have_conan();
    results[2] = have_conda();
    results[3] = have_npm();
    results[4] = have_uv();

    let mut incomplete = false;

    for i in 0..results.len() {
        match results[i] {
            CheckStatus::Success => {
                info!("HRI09 passed ✅");
                return CheckStatus::Success
            }
            CheckStatus::Incomplete => {
                incomplete = true;
            }
            _ => {}
        }
    }

    match incomplete
    {
        true => {
            warn!("HRI09 incomplete ⏳");
            CheckStatus::Incomplete
    }
        false => {
            error!("none found");
            error!("HRI09 failed ❌");
            CheckStatus::Failure
        }
    }
}

fn have_cargo() -> CheckStatus {
    have_one_config_one_lock( CARGO_TOML, CARGO_LOCK)
}

fn have_conan() -> CheckStatus {
    have_two_config_one_lock( CONANFILE_PY, CONANFILE_TXT, CONAN_LOCK )
}


fn have_conda() -> CheckStatus {
    have_two_config_one_lock( ENVIRONMENT_YML, ENVIRONMENT_YAML, CONDA_LOCK )
}

fn have_npm() -> CheckStatus {
    have_one_config_one_lock( PACKAGE_JSON, PACKAGE_LOCK_JSON)
}

fn have_uv() -> CheckStatus {
    have_two_config_one_lock( UV_TOML, PYPROJECT_TOML, UV_LOCK)
}


fn have_one_config_one_lock( config: &str, lock: &str ) -> CheckStatus {
    let have_config = file_exists(config);
    let have_lock = file_exists(lock);

    if have_config {
        info!("found {}", config);
    }
    if have_lock {
        info!("found {}", lock);
    }

    if have_config {
        if have_lock {
            return CheckStatus::Success;
        }
        warn!("lockfile missing: {}", lock);
        return CheckStatus::Incomplete;
    }

    CheckStatus::Failure
}

fn have_two_config_one_lock( config_a: &str, config_b: &str, lock: &str ) -> CheckStatus {
    let have_config_a = file_exists(config_a);
    let have_config_b = file_exists(config_b);
    let have_lock = file_exists(lock);

    if have_config_a {
        info!("found {}", config_a);
    }
    if have_config_b {
        info!("found {}", config_b);
    }
    if have_lock {
        info!("found {}", lock);
    }

    if have_config_a || have_config_b {
        if have_lock {
            return CheckStatus::Success;
        }
        warn!("lockfile missing: {}", lock);
        return CheckStatus::Incomplete;
    }

    CheckStatus::Failure
}

fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
