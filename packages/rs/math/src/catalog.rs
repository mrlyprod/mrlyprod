use mrly_name::named_enum;
use serde::{Deserialize, Serialize};

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum Design {
        Carpet => "Carpet",
        Net => "Net",
        Htree => "Htree",
        Vtree => "Vtree",
        Void => "Void",
        Xtree => "Xtree",
        Ytree => "Ytree",
        Ztree => "Ztree",
        Point => "Point",
        Dust => "Dust",
        Hline => "Hline",
        Vline => "Vline",
        Star => "Star",
        Xline => "Xline",
        Yline => "Yline",
        Zline => "Zline",
    }
}

pub const CLASSICS_2D: [Design; 5] = [
    Design::Carpet,
    Design::Net,
    Design::Htree,
    Design::Vtree,
    Design::Void,
];

pub const CLASSICS_3D: [Design; 6] = [
    Design::Carpet,
    Design::Net,
    Design::Xtree,
    Design::Ytree,
    Design::Ztree,
    Design::Void,
];

pub const ANTIS_2D: [Design; 5] = [
    Design::Point,
    Design::Dust,
    Design::Hline,
    Design::Vline,
    Design::Star,
];

pub const ANTIS_3D: [Design; 6] = [
    Design::Point,
    Design::Dust,
    Design::Xline,
    Design::Yline,
    Design::Zline,
    Design::Star,
];

pub fn classics(dimension: usize) -> Vec<Design> {
    match dimension {
        3 => CLASSICS_3D.to_vec(),
        _ => CLASSICS_2D.to_vec(),
    }
}

pub fn antis(dimension: usize) -> Vec<Design> {
    match dimension {
        3 => ANTIS_3D.to_vec(),
        _ => ANTIS_2D.to_vec(),
    }
}
