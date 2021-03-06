#[macro_use]
extern crate speedy_derive;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod utils;

mod args;
mod raw_data;
mod perf_group;
mod perf_arch;
mod archive;
mod execution_queue;
mod kallsyms;
mod ps;
mod stack_reader;
mod metadata;
mod mount_info;
mod profiler;
mod cmd_record;
#[cfg(feature = "inferno")]
mod cmd_flamegraph;
mod cmd_collate;
mod cmd_metadata;

use std::env;
use std::error::Error;
use std::process::exit;
use structopt::StructOpt;

fn main_impl() -> Result< (), Box< Error >  > {
    if env::var( "RUST_LOG" ).is_err() {
        env::set_var( "RUST_LOG", "nperf=info" );
    }

    #[cfg(feature = "env_logger")]
    env_logger::init();

    let opt = args::Opt::from_args();
    match opt {
        args::Opt::Record( args ) => {
            if args.profiler_args.panic_on_partial_backtrace {
                warn!( "Will panic on partial backtraces!" );
                if env::var( "RUST_BACKTRACE" ).is_err() {
                    env::set_var( "RUST_BACKTRACE", "1" );
                }
            }

            cmd_record::main( args )?;
        },
        #[cfg(feature = "inferno")]
        args::Opt::Flamegraph( args ) => {
            cmd_flamegraph::main( args )?;
        },
        args::Opt::Collate( args ) => {
            cmd_collate::main( args )?;
        },
        args::Opt::Metadata( args ) => {
            cmd_metadata::main( args )?;
        }
    }

    Ok(())
}

fn main() {
    if let Err( error ) = main_impl() {
        eprintln!( "error: {}", error );
        exit( 1 );
    }
}
