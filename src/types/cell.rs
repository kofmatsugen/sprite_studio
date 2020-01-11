use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    map_id: usize,
    cell_id: usize,
}

impl Cell {
    pub fn map_id(&self) -> usize {
        self.map_id
    }

    pub fn cell_id(&self) -> usize {
        self.cell_id
    }
}

pub struct CellBuilder {
    map_id: usize,
    cell_id: usize,
}

impl CellBuilder {
    pub fn new(map_id: usize, cell_id: usize) -> Self {
        CellBuilder { map_id, cell_id }
    }

    pub fn build(self) -> Cell {
        Cell {
            map_id: self.map_id,
            cell_id: self.cell_id,
        }
    }
}
