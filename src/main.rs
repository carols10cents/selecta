extern crate getopts;

use std::os;

static VERSION: &'static str = "0.0.1-shipoopi";

fn main () {
    let args = os::args();
    let ref program = args[0];

    let opts = [
        getopts::optflag("h", "help", "Show this message"),
        getopts::optflag("v", "version", "Show version"),
    ];

    let opt_matches = match getopts::getopts(args.tail(), &opts) {
        Ok(m)  => m,
        Err(f) => { panic!(f.to_string()) }
    };

    if opt_matches.opt_present("help") {
        println!("selecta {}", VERSION);
        println!("");
        let usage = format!("Usage: {} [options]", program);
        println!("{}", getopts::usage(usage.as_slice(), &opts));
        return;
    }

    if opt_matches.opt_present("version") {
       println!("selecta version: {}", VERSION);
       return;
   }
}
