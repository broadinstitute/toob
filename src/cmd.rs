mod map_variants;

use crate::cli::Args;
use crate::cmd::map_variants::map_variants;
use crate::error::Error;
use std::collections::BTreeMap;

type FuncBox = Box<dyn Fn(Args) -> Result<(), Error>>;
pub(crate) struct Cmds {
    cmds: BTreeMap<String, FuncBox>,
}

impl Cmds {
    pub(crate) fn new() -> Cmds {
        let mut cmds: BTreeMap<String, FuncBox>  = BTreeMap::new();
        cmds.insert(names::MAP_VARIANTS.to_string(), Box::new(map_variants));
        Cmds { cmds }
    }
    pub(crate) fn get(&self, cmd: &str) -> Option<&FuncBox> {
        self.cmds.get(cmd)
    }
    pub(crate) fn known_cmds_are(&self) -> String {
        let mut cmd_iter = self.cmds.iter();
        if let Some((cmd, _)) = cmd_iter.next() {
            let mut list: String = format!("Known commands are: '{}'", cmd);
            for (cmd, _) in cmd_iter {
                list.push_str(", '");
                list.push_str(cmd);
                list.push('\'')
            }
            list.push('.');
            list
        } else {
            "No commands available.".to_string()
        }
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

