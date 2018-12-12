// Copyright 2017-2018 Kisio Digital and/or its affiliates.
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see
// <http://www.gnu.org/licenses/>.

#[macro_use]
extern crate log;

use failure::ResultExt;
use osm_tools::Result;
use osm_utils::make_osm_reader;
use osm_utils::poi::{pois, PoiConfig};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "extract_osm_pois",
    about = "Extract POIs from OSM.",
    rename_all = "kebab-case"
)]
struct Opt {
    /// OSM PBF file.
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,

    /// POIs configuration.
    #[structopt(short = "c", long, parse(from_os_str))]
    poi_config: Option<PathBuf>,

    /// Output poi file.
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

fn run() -> Result<()> {
    info!("Launching extract_osm_pois.");

    let opt = Opt::from_args();

    let matcher = match opt.poi_config {
        None => PoiConfig::default(),
        Some(path) => {
            let r = std::fs::File::open(&path)
                .with_context(|_| format!("Error while opening configuration file {:?}", path))?;
            PoiConfig::from_reader(r).unwrap()
        }
    };
    info!("Extracting pois from osm");

    let mut osm_reader = make_osm_reader(&opt.input)?;
    let _pois = pois(&mut osm_reader, &matcher);

    Ok(())
}

fn main() {
    env_logger::init();
    if let Err(err) = run() {
        for cause in err.iter_chain() {
            eprintln!("{}", cause);
        }
        std::process::exit(1);
    }
}
