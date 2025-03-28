//
//  Quality Cluster HRI10 - Obey HRI01-HRI09 ;)
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

pub fn run(results: &[CheckStatus]) -> i32 {
    info!("checking HRI10 (Obey HRI01-HRI-09 😊)");

    let mut success = true;

    for (i, item ) in results.iter().enumerate() {
        let id = i + 1;
        match item {
            CheckStatus::Success => {
                info!("summary: HRI{id:02} passed ✅");
            }
            CheckStatus::Incomplete => {
                info!("summary: HRI{id:02} incomplete ⏳");
                success = false;
            }
            CheckStatus::Failure => {
                info!("summary: HRI{id:02} failed ❌");
                success = false;
            }
            CheckStatus::NotApplicable => {
                info!("summary: HRI{id:02} not applicable");
            }
        }
    }

    match success
    {
        true => {
            info!("summary: HRI10 passed ✅");
            0 // will be used as program exit code
        },
        false => {
            error!("summary: HRI10 failed ❌");
            1 // will be used as program exit code
        }
    }
}
