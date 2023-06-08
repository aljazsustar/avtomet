pub struct Sort {
    pub tip_sort: SortDirection,
    pub presort: SortCategory,
    pub subsort: SortCategory,
    pub subtipsort: SortDirection,
    pub stran: u8,
}

impl Default for Sort {
    fn default() -> Self {
        Self {
            tip_sort: SortDirection::DESC,
            presort: SortCategory::PRICE,
            subsort: SortCategory::PRICE,
            subtipsort: SortDirection::DESC,
            stran: Default::default(),
        }
    }
}

pub enum SortDirection {
    ASC,
    DESC,
}

pub enum SortCategory {
    PRICE,
    NEWEST,
}
