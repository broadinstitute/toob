mod map_variants;

use std::collections::BTreeMap;
use crate::cli::Args;
use crate::cmd::map_variants::map_variants;
use crate::error::Error;

type FuncBox = Box<dyn Fn(Args) -> Result<(), Error>>;
pub struct Cmds {
    cmds: BTreeMap<String, FuncBox>,
}

impl Cmds {
    pub fn new() -> Cmds {
        let mut cmds: BTreeMap<String, FuncBox>  = BTreeMap::new();
        cmds.insert(names::MAP_VARIANTS.to_string(), Box::new(map_variants));
        Cmds { cmds }
    }
    pub fn get(&self, cmd: &str) -> Option<&FuncBox> {
        self.cmds.get(cmd)
    }
}

impl Default for Cmds {
    fn default() -> Self {
        Cmds::new()
    }
}

mod names {
    pub(crate) const MAP_VARIANTS: &str = "map_variants";
}

