use crate::{soa::MakeDefault, U32Id};
use std::marker::PhantomData;

pub struct IdStruct<T, TMarker> {
    phantom: PhantomData<TMarker>,
    items: Vec<T>,
}

impl<T, TMarker> IdStruct<T, TMarker> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
            items: Vec::new(),
        }
    }

    pub fn retain(&mut self, id: U32Id<TMarker>, value: T)
    where
        T: MakeDefault<T>,
    {
        self.retain_mk::<T>(id, value)
    }

    pub fn retain_mk<TMakeDefault: MakeDefault<T>>(&mut self, id: U32Id<TMarker>, value: T) {
        ensure_size::<_, TMakeDefault>(&mut self.items, (id.to_u32() + 1) as usize);
        self.items[id.to_u32() as usize] = value;
    }

    pub fn release(&mut self, id: U32Id<TMarker>)
    where
        T: MakeDefault<T>,
    {
        self.release_mk::<T>(id)
    }

    pub fn release_mk<TMakeDefault: MakeDefault<T>>(&mut self, id: U32Id<TMarker>) {
        self.items[id.to_u32() as usize] = TMakeDefault::make_default()
    }
}

fn ensure_size<T, TMakeDefault: MakeDefault<T>>(items: &mut Vec<T>, desired_size: usize) {
    while items.len() < desired_size {
        items.push(TMakeDefault::make_default())
    }
}

impl<T, TMarker> Default for IdStruct<T, TMarker> {
    fn default() -> Self {
        Self::new()
    }
}
