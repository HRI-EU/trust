//
//  Quality Cluster HRI08 - License compliance
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
use log::{error, info};
use regex::Regex;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

const CONFIGFILE: &str = ".gitlab-ci.yml";

pub fn run() -> CheckStatus {
    info!("checking HRI08 (License compliance)");

    // We use regular expressions instead of a simple string comparison in order to handle:
    //    * varying hostnames
    //    * component inclusions that have been commented out

    let re = Regex::new(r"^\s+-\s+component:\s[A-Za-z0-9._\-/$]+/[TECHtech_TEAMteam]+/ci/autocompliance/(autocompliance@.+$)").unwrap();

    match fs::metadata(CONFIGFILE) {
        Ok(_) => match File::open(CONFIGFILE) {
            Ok(fh) => {
                let lines = BufReader::new(fh).lines();

                for line in lines.map_while(Result::ok) {
                    if let Some(caps) = re.captures(&line) {
                        info!("found: {}", caps.get(1).unwrap().as_str());
                        info!("HRI08 passed ✅");
                        return CheckStatus::Success;
                    }
                }

                error!("CI component 'autocompliance' not found");
                error!("HRI08 failed ❌");
                CheckStatus::Failure
            }
            Err(e) => {
                error!("{}", e);
                error!("{}: unable to read file", CONFIGFILE);
                error!("HRI08 failed ❌");
                CheckStatus::Failure
            }
        },
        Err(_) => {
            error!("no CI/CD pipelines found");
            error!("HRI08 failed ❌");
            CheckStatus::Failure
        }
    }
}
