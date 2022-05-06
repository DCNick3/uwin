use std::cmp::min;
use std::marker::PhantomData;
use tracing::trace;

pub trait Handle<Ctx = ()>: Copy {
    fn to_index(self, ctx: &Ctx) -> usize;
    fn from_index(index: usize, ctx: &Ctx) -> Self;
}

pub struct HandleTable<Ctx, H: Handle<Ctx>, V, const RESIZE: bool> {
    ctx: Ctx,
    table: Vec<Option<V>>,
    start_free_search_idx: usize,
    phantom: PhantomData<H>,
}

impl<Ctx, H: Handle<Ctx>, V, const RESIZE: bool> HandleTable<Ctx, H, V, RESIZE> {
    pub fn new(ctx: Ctx, initial_handle_count: usize) -> Self {
        let mut table = Vec::new();
        table.resize_with(initial_handle_count, || None);
        Self {
            ctx,
            table,
            start_free_search_idx: 0,
            phantom: Default::default(),
        }
    }

    pub fn put(&mut self, object: V) -> H {
        for (i, v) in self
            .table
            .iter_mut()
            .enumerate()
            .skip(self.start_free_search_idx as usize)
        {
            if v.is_none() {
                *v = Some(object);
                self.start_free_search_idx = i;
                return H::from_index(i, &self.ctx);
            }
        }

        if RESIZE {
            // no free slot found, need  to expand the table
            trace!(
                "Could not find free slots in handle table, doubling the size (current size = {})",
                self.table.len()
            );

            self.table.resize_with(self.table.len() * 2, || None);

            self.put(object)
        } else {
            panic!(
                "Could not find free slots in handle table (size = {})",
                self.table.len()
            )
        }
    }

    pub fn find(&self, handle: H) -> Option<&V> {
        self.table
            .get(handle.to_index(&self.ctx))
            .map(|f| f.as_ref())?
    }

    pub fn find_mut(&mut self, handle: H) -> Option<&mut V> {
        self.table
            .get_mut(handle.to_index(&self.ctx))
            .map(|f| f.as_mut())?
    }

    pub fn remove(&mut self, handle: H) -> Option<V> {
        let index = handle.to_index(&self.ctx);
        let obj = self.table.get_mut(index)?;
        obj.take().map(|obj| {
            self.start_free_search_idx = min(self.start_free_search_idx, index);
            obj
        })
    }

    pub fn capacity(&self) -> usize {
        self.table.len()
    }
}

#[cfg(test)]
mod test {
    use crate::Handle;

    impl Handle for u32 {
        fn to_index(self, _: &()) -> usize {
            let handle: usize = self.try_into().unwrap();
            (handle >> 2) - 10
        }

        fn from_index(index: usize, _: &()) -> Self {
            let index = (index + 10) << 2;
            index.try_into().unwrap()
        }
    }

    struct Obj(u8);

    #[test]
    fn test() {
        type HandleTable = super::HandleTable<(), u32, Obj, true>;
        let mut table = HandleTable::new((), 16);

        assert_eq!(table.table.len(), 16);

        let mut handles = Vec::new();
        for i in 0..17 {
            handles.push(table.put(Obj(i)));
        }

        println!("Allocated handles: {:?}", handles);

        for &handle in handles.iter() {
            assert!(table.find(handle).is_some());
        }

        // handle table was resized!
        assert_eq!(table.table.len(), 32);

        assert!(table.remove(handles[0]).is_some());

        // handle was reused!
        assert_eq!(table.put(Obj(18)), handles[0]);
    }
}
