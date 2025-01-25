//
//  Quality Cluster HRI03 - Mind copyright headers
//
//  Copyright (c) Honda Research Institute Europe GmbH
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
use std::str;
use walkdir::{DirEntry, WalkDir};

const FILE_EXTENSIONS: [&'static str; 15] = [
    ".bash", ".c", ".cpp", ".go", ".h", ".hpp", ".hxx", ".inc", ".js", ".m", ".py", ".rs", ".sh",
    ".ts", ".tsx",
];

pub fn run() -> CheckStatus {
    info!("checking HRI03 (Mind copyright headers)");

    let status = worker("src");

    match status {
        CheckStatus::Success => {
            info!("HRI03 passed ✅")
        }
        CheckStatus::Incomplete => {
            warn!("HRI03 incomplete ⏳")
        }
        CheckStatus::Failure => {
            error!("HRI03 failed ❌")
        }
        CheckStatus::NotApplicable => {
            info!("HRI03 not applicable")
        }

        // should not happen
        CheckStatus::NotImplemented => {
            info!("HRI03 not implemented")
        }
    }

    status
}

fn worker(path: &str) -> CheckStatus {
    let mut found_good = false;
    let mut found_incomplete = false;
    let mut found_bad = false;

    match fs::metadata(path) {
        Ok(_) => {
            WalkDir::new(path)
                .sort_by_file_name()
                .into_iter()
                .filter_entry(|e| is_of_interest(&e))
                .filter_map(|v| v.ok())
                .for_each(|e| {
                    if e.file_type().is_file() {
                        match process_file(&e) {
                            CheckStatus::Success => found_good = true,
                            CheckStatus::Incomplete => found_incomplete = true,
                            CheckStatus::Failure => found_bad = true,
                            _ => {}
                        }
                    }
                });

            if !found_bad && !found_incomplete {
                CheckStatus::Success
            } else if !found_good && !found_incomplete {
                CheckStatus::Failure
            } else {
                CheckStatus::Incomplete
            }
        }
        Err(e) => {
            error!("{}", e);
            error!("'{path}' subdirectory missing or not readable");
            CheckStatus::NotApplicable
        }
    }
}

fn is_of_interest(entry: &DirEntry) -> bool {
    // We skip hidden files and directories (starting with ".").
    // For files, we also only consider known file extensions.
    // Everything else, such as symlinks, is ignored.

    let ft = entry.file_type();

    if ft.is_dir() {
        is_not_hidden(entry)
    } else if ft.is_file() {
        is_not_hidden(entry) && has_source_ext(entry)
    } else {
        false
    }
}

/* from https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
published under CC0 (aka public domain) */
fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn has_source_ext(entry: &DirEntry) -> bool {
    for candidate in FILE_EXTENSIONS {
        if entry.file_name().to_str().unwrap().ends_with(candidate) {
            return true;
        }
    }

    false
}

fn process_file(entry: &DirEntry) -> CheckStatus {
    let path = entry.path();

    match fs::read_to_string(path) {
        Ok(content) => {
            let c_low = content.to_lowercase();
            let header_found =
                c_low.contains("copyright") || c_low.contains("(c)") || c_low.contains("©");

            let spdx_found = content.contains("SPDX-License-Identifier:");

            if header_found && spdx_found {
                info!("copyright and SPDX info found: {}", path.display());
                CheckStatus::Success
            } else if header_found {
                warn!("copyright found, SPDX missing: {}", path.display());
                CheckStatus::Incomplete
            } else if spdx_found {
                warn!("copyright missing, SPDX found: {}", path.display());
                CheckStatus::Incomplete
            } else {
                error!("copyright and SPDX ID missing: {}", path.display());
                CheckStatus::Failure
            }
        }
        Err(e) => {
            error!("{}", e);
            error!("file not readable: {}", path.display());
            CheckStatus::Failure
        }
    }
}
