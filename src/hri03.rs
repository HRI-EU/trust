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
//

use log::{error, info, warn};
use std::fs;
use std::str;
use walkdir::{DirEntry, WalkDir};

const FILE_EXTENSIONS: [&'static str; 15] = [
    "bash", "c", "cpp", "go", "h", "hpp", "hxx", "inc", "js", "m", "py", "rs", "sh", "ts", "tsx",
];

pub fn run() {
    info!("checking HRI03 (Mind copyright headers)");

    match worker("src") {
        true => info!("HRI03 passed ✅"),
        false => error!("HRI03 failed ❌"),
    }
}

fn worker(path: &str) -> bool {
    let mut overall_result: bool = true;

    match fs::metadata(path) {
        Ok(_) => {
            WalkDir::new("src")
                .sort_by_file_name()
                .into_iter()
                .filter_entry(|e| is_of_interest(&e))
                .filter_map(|v| v.ok())
                .for_each(|e| {
                    if e.file_type().is_file() {
                        let file_result = process_file(&e);
                        if file_result == false {
                            overall_result = false;
                        }
                    }
                });

            overall_result
        }
        Err(e) => {
            error!("{}", e);
            error!("'src' subdirectory missing or not readable");
            false
        }
    }
}

fn is_of_interest(entry: &DirEntry) -> bool {
    is_not_hidden(entry) && has_source_ext(entry)
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

fn process_file(entry: &DirEntry) -> bool {
    let path = entry.path();
    // debug!("processing file: {}", path.display());

    match fs::read_to_string(path) {
        Ok(content) => {
            let c_low = content.to_lowercase();

            if c_low.contains("copyright") || c_low.contains("(c)") || c_low.contains("©") {
                info!("copyright header found in: {}", path.display());
                true
            }
            else {
                warn!("copyright header missing: {}", path.display());
                false
            }
        }
        Err(e) => {
            error!("{}", e);
            error!("file not readable: {}", path.display());
            false
        }
    }
}
