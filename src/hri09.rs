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

pub fn run() -> CheckStatus {
    info!("checking HRI09 (Use modern build/packaging system)");

    let mut results: [CheckStatus; 4] = [CheckStatus::NotApplicable; 4];

    results[0] = have_conan();
    results[1] = have_cargo();
    results[2] = have_conda();
    results[3] = have_uv();

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

fn have_conan() -> CheckStatus {
    let conanfile_py = file_exists(CONANFILE_PY);
    let conanfile_txt = file_exists(CONANFILE_TXT);
    let conan_lock = file_exists(CONAN_LOCK);

    if conanfile_py {
        info!("found {}", CONANFILE_PY);
    }
    if conanfile_txt {
        info!("found {}", CONANFILE_TXT);
    }
    if conan_lock {
        info!("found {}", CONAN_LOCK);
    }

    if conanfile_py || conanfile_txt {
        if conan_lock {
            return CheckStatus::Success;
        }
        warn!("lockfile missing: {}", CONAN_LOCK);
        return CheckStatus::Incomplete;
    }

    CheckStatus::Failure
}

fn have_cargo() -> CheckStatus {
    let cargo_toml = file_exists(CARGO_TOML);
    let cargo_lock = file_exists(CARGO_LOCK);

    if cargo_toml {
        info!("found {}", CARGO_TOML);
    }
    if cargo_lock {
        info!("found {}", CARGO_LOCK);
    }

    if cargo_toml {
        if cargo_lock {
            return CheckStatus::Success;
        }
        warn!("lockfile missing: {}", CARGO_LOCK);
        return CheckStatus::Incomplete;
    }

    CheckStatus::Failure
}

fn have_conda() -> CheckStatus {
    let environment_yml = file_exists(ENVIRONMENT_YML);
    let environment_yaml = file_exists(ENVIRONMENT_YAML);
    let conda_lock = file_exists(CONDA_LOCK);

    if environment_yml {
        info!("found {}", ENVIRONMENT_YML);
    }
    if environment_yaml {
        info!("found {}", ENVIRONMENT_YAML);
    }
    if conda_lock {
        info!("found {}", CONDA_LOCK);
    }

    if environment_yml || environment_yaml {
        if conda_lock {
            return CheckStatus::Success;
        }
        warn!("lockfile missing: {}", CONDA_LOCK);
        return CheckStatus::Incomplete;
    }

    CheckStatus::Failure
}

fn have_uv() -> CheckStatus {
    let uv_toml = file_exists(UV_TOML);
    let pyproject_toml = file_exists(PYPROJECT_TOML);
    let uv_lock = file_exists(UV_LOCK);

    if uv_toml {
        info!("found {}", UV_TOML);
    }
    if pyproject_toml {
        info!("found {}", PYPROJECT_TOML);
    }
    if uv_lock {
        info!("found {}", UV_LOCK);
    }

    if uv_toml || pyproject_toml {
        if uv_lock {
            return CheckStatus::Success;
        }
        warn!("lockfile missing: {}", UV_LOCK);
        return CheckStatus::Incomplete;
    }

    CheckStatus::Failure
}

fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}
