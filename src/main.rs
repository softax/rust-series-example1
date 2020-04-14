//! Conway's Game of Life main function. Program requires three parameters, 
//! if you are running it with help of cargo then write on terminal: 
//! 
//! cargo run -- -g 50 -s 20 -t 0.3
//!
//! or 
//! game_of_life -g 50 -s 20 -t 0.3
//! 
//! Crate modue Grid is used to perform actual simulation. Ase it as follows:
//!
//! ```
//! let mut g = Grid::new(grid_size, threshold);
//! g.update();
//! g.save( "file_name", &ImageExportConfig::default());        
//! ```

use clap::{App, Arg};
use game_of_life::grid::Grid;
use game_of_life::save::ImageExportConfig;

use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let matches = App::new("Game of life")
        .arg(
            Arg::with_name("grid size")
                .help("Size of rectangular grid on which Conway's game of life unfolds")
                .required(true)
                .short("gs")
                .value_name("GRID SIZE")
                .long("grid-size")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("steps")
                .help("Number of simulation steps")
                .required(true)
                .short("s")
                .long("steps")
                .value_name("STEPS NUMBER")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("threshold")
                .help("Random initialization threshold")
                .required(true)
                .short("t")
                .value_name("THRESHOLD")
                .long("threshold")
                .takes_value(true),
        )
        .get_matches();

    let grid_size: usize = matches
        .value_of("grid size")
        .unwrap_or_default()
        .parse::<usize>()
        .expect("size has to be numeric!");
    let steps: usize = matches
        .value_of("steps")
        .unwrap_or_default()
        .parse::<usize>()
        .expect("steps has to be numeric!");
    let threshold: f64 = matches
        .value_of("threshold")
        .unwrap_or_default()
        .parse::<f64>()
        .expect("threshold has to be floatin range 0.0-1.1!");

    if threshold < 0.0 || threshold > 1.0 {
        panic!("Treshold should be value in range: 0.0 - 1.0");
    }

    let pb = ProgressBar::new(steps as u64);

    pb.set_style(ProgressStyle::default_bar()
        .template("Simulation steps: [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
        .progress_chars("#>-"));

    let mut g = Grid::new(grid_size, threshold);
    
    for no in 0..steps {
        g.save(
            &format!("output/game_of_life_{:05}.png", no),
            &ImageExportConfig::default(),
        )
        .expect("Error while save file!");
        g.update();
        pb.set_position(no as u64);
    }
    pb.finish_with_message("simulation finished");
}
