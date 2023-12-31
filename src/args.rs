use bevy::prelude::Resource;
use clap::Parser;


#[derive(Parser, Resource, Debug, Clone)]
pub struct Args {
    /// runs the game in synctest mode
    #[clap(long)]
    pub synctest: bool,
    /// sets a custom input delay
    #[clap(long, default_value = "2")] // new
    pub input_delay: usize, // new
}

