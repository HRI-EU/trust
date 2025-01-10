//
//  Trust - A minimal checker for DevSecOps best-practices
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

mod hri01;
mod hri02;
mod hri03;
mod hri04;
mod hri05;
mod hri06;
mod hri07;
mod hri08;
mod hri09;
mod hri10;

use clap::Parser;
use log::info;
use simple_logger::SimpleLogger;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {}

// Cargo passes settings from Cargo.toml as env. variable to compiler
const TRUST_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy)]
#[derive(Clone)]
enum CheckStatus {
    Success,
    Incomplete,
    Failure,
    NotApplicable,
    NotImplemented,
}

fn main() {
    SimpleLogger::new().env().init().unwrap();

    Args::parse();

    show_splash();
    run_checks();
}

fn show_splash() {
    // ASCII art created with https://budavariam.github.io/asciiart-text
    // font: ANSI Shadow

    info!("");
    info!("████████╗██████╗ ██╗   ██╗███████╗████████╗");
    info!("╚══██╔══╝██╔══██╗██║   ██║██╔════╝╚══██╔══╝");
    info!("   ██║   ██████╔╝██║   ██║███████╗   ██║");
    info!("   ██║   ██╔══██╗██║   ██║╚════██║   ██║");
    info!("   ██║   ██║  ██║╚██████╔╝███████║   ██║");
    info!("   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝");
    info!("   DevSecOps Best-Practices Scanner {TRUST_VERSION}");
    info!("");
}


fn run_checks() {
    let mut results: [CheckStatus; 9] = [ CheckStatus::NotApplicable; 9 ];

    info!("");
    results[0] = hri01::run();
    info!("");
    results[1] = hri02::run();
    info!("");
    results[2] = hri03::run();
    info!("");
    results[3] = hri04::run();
    info!("");
    results[4] = hri05::run();
    info!("");
    results[5] = hri06::run();
    info!("");
    results[6] = hri07::run();
    info!("");
    results[7] = hri08::run();
    info!("");
    results[8] = hri09::run();
    info!("");
    hri10::run( &results );
}

