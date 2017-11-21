extern crate fuse;

use std::env;
use fuse::Filesystem;

struct MulaFs;

impl Filesystem for MulaFs {

}

fn main() {
    let mountpoint = match env::args()
}