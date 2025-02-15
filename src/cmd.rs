mod vars2exac;
mod json2tsv;
mod parse_allele_reg;

use crate::cli::Args;
use crate::cmd::vars2exac::vars2exac;
use crate::error::Error;
use std::collections::BTreeMap;

type FuncBox = Box<dyn Fn(Args) -> Result<(), Error>>;
pub(crate) struct Cmds {
    cmds: BTreeMap<String, FuncBox>,
}

impl Cmds {
    pub(crate) fn new() -> Cmds {
        let mut cmds: BTreeMap<String, FuncBox>  = BTreeMap::new();
        cmds.insert(names::VARS2EXAC.to_string(), Box::new(vars2exac));
        cmds.insert(names::JSON2TSV.to_string(), Box::new(json2tsv::json2tsv));
        cmds.insert(names::PARSE_ALLELE_REG.to_string(),
                    Box::new(parse_allele_reg::parse_allele_reg));
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
    pub(crate) const VARS2EXAC: &str = "vars2exac";
    pub(crate) const JSON2TSV: &str = "json2tsv";
    pub(crate) const PARSE_ALLELE_REG: &str = "parse_allele_reg";
}

