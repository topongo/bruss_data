use tt::AreaType;
use AreaType::{U, E};

use crate::InArea;
use std::collections::HashMap;


#[derive(Default)]
pub struct AreaHelper<T: InArea> {
    urban: HashMap<u16, T>,
    extra: HashMap<u16, T>,
}


impl<T: InArea> AreaHelper<T> {
    pub fn new() -> Self { 
        Self { urban: HashMap::new(), extra: HashMap::new() }
    }

    pub fn insert(&mut self, s: T) {
        self.get_mut(s.ty()).insert(s.id(), s);
    }

    pub fn urban(&self) -> &HashMap<u16, T> {
        self.get(U)
    }

    pub fn urban_mut(&mut self) -> &mut HashMap<u16, T> {
        self.get_mut(U)
    }
    
    pub fn extra(&self) -> &HashMap<u16, T> { 
        self.get(E)
    }
    
    pub fn extra_mut(&mut self) -> &mut HashMap<u16, T> { 
        self.get_mut(E)
    }

    pub fn get(&self, ty: AreaType) -> &HashMap<u16, T> {
        match ty {
            AreaType::E => &self.extra,
            AreaType::U => &self.urban,
        }
    }

    pub fn get_mut(&mut self, ty: AreaType) -> &mut HashMap<u16, T> {
        match ty {
            AreaType::E => &mut self.extra,
            AreaType::U => &mut self.urban,
        }
    }
}

