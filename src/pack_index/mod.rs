use smallstring::SmallString;
use minidom::{Element, Error, ErrorKind};

use parse::{attr_map, child_text, assert_root_name, DEFAULT_NS, FromElem};

pub mod network;

#[derive(Debug, Clone)]
pub struct PdscRef {
    pub url: String,
    pub vendor: SmallString,
    pub name: SmallString,
    pub version: SmallString,
    pub date: Option<String>,
    pub deprecated: Option<String>,
    pub replacement: Option<String>,
    pub size: Option<String>,
}

#[derive(Debug)]
pub struct Pidx {
    pub url: String,
    pub vendor: SmallString,
    pub date: Option<String>,
}


#[derive(Debug)]
pub struct Vidx {
    pub vendor: String,
    pub url: String,
    pub timestamp: Option<String>,
    pub pdsc_index: Vec<PdscRef>,
    pub vendor_index: Vec<Pidx>,
}

impl FromElem for PdscRef {
    fn from_elem(e: &Element) -> Result<Self, Error> {
        assert_root_name(e, "pdsc")?;
        Ok(Self {
            url: attr_map(e, "url", "pdsc")?,
            vendor: attr_map(e, "vendor", "pdsc")?,
            name: attr_map(e, "name", "pdsc")?,
            version: attr_map(e, "version", "pdsc")?,
            date: attr_map(e, "date", "pdsc").ok(),
            deprecated: attr_map(e, "deprecated", "pdsc").ok(),
            replacement: attr_map(e, "replacement", "pdsc").ok(),
            size: attr_map(e, "size", "pdsc").ok(),
        })
    }
}


impl FromElem for Pidx {
    fn from_elem(e: &Element) -> Result<Self, Error> {
        assert_root_name(e, "vidx")?;
        Ok(Self {
            url: attr_map(e, "url", "pidx")?,
            vendor: attr_map(e, "vendor", "pidx")?,
            date: attr_map(e, "date", "pidx").ok(),
        })
    }
}

impl FromElem for Vidx {
    fn from_elem(root: &Element) -> Result<Self, Error> {
        assert_root_name(root, "index")?;
        let vendor = child_text(root, "vendor", "index")?;
        let url = child_text(root, "url", "index")?;
        Ok(Vidx {
            vendor,
            url,
            timestamp: root.get_child("timestamp", DEFAULT_NS).map(Element::text),
            vendor_index: root.get_child("vindex", DEFAULT_NS)
                .map(Element::children)
                .map(Pidx::vec_from_children)
                .unwrap_or_default(),
            pdsc_index: root.get_child("pindex", DEFAULT_NS)
                .map(Element::children)
                .map(PdscRef::vec_from_children)
                .unwrap_or_default(),
        })
    }
}
