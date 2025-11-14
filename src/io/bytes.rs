use std::{
    alloc::{Layout, alloc, dealloc, realloc},
    ops::{Index, IndexMut, RangeFrom, RangeFull, RangeTo, RangeToInclusive},
};

pub struct BytesHandler {
    ptr: *mut u8,
    length: usize,
    capacity: usize,
}

impl BytesHandler {
    pub fn new() -> Self {
        Self::with_capacity(512)
    }

    pub fn with_capacity(n: usize) -> Self {

        Self {
            ptr: unsafe { alloc(Layout::array::<u8>(n).unwrap()) },
            length: 0,
            capacity: n,
        }    
    }

    pub fn cap(&self) -> usize {
        self.capacity
    }

    pub fn as_slice(&self) -> &[u8] {
        self.as_ref()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn drain<T>(&mut self, range: T) -> Option<BytesHandler>
    where
        T: std::ops::RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(e) => *e,
            std::ops::Bound::Excluded(e) => *e + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(e) => *e,
            std::ops::Bound::Excluded(e) => *e - 1,
            std::ops::Bound::Unbounded => self.length - 1,
        };

        if start >= self.length {
            return None;
        }

        unsafe {
            let size = (end - start + 1).min(self.length);
            let cap = size.div_ceil(512) * 512;
            let new_ptr = alloc(Layout::array::<u8>(cap).ok()?);

            std::ptr::copy_nonoverlapping(
                (self.ptr as usize + (start * std::mem::size_of::<u8>())) as *const u8,
                new_ptr,
                size,
            );

            let resp = Some(BytesHandler {
                ptr: new_ptr,
                length: size,
                capacity: cap,
            });
            let self_ptr = self.ptr;

            let self_capacity = self.capacity;

            self.length -= size;
            self.capacity = self.length.div_ceil(512) * 512;

            self.ptr = alloc(Layout::array::<u8>(self.capacity).ok()?);
            std::ptr::copy_nonoverlapping(self_ptr as *const u8, self.ptr, start);

            std::ptr::copy_nonoverlapping(
                (self_ptr as usize + ((end + 1) * std::mem::size_of::<u8>())) as *const u8,
                (self.ptr as usize + ((start + 1) * std::mem::size_of::<u8>())) as *mut u8,
                size,
            );

            dealloc(self_ptr, Layout::array::<u8>(self_capacity).ok()?);

            resp
        }
    }

    pub fn extend(&mut self, slice: &[u8]) {
        if slice.len() > self.capacity - self.length {
            let ad = (slice.len() - self.capacity - self.length).div_ceil(512) * 512;
            self.realloc(ad + self.capacity);
            self.capacity += ad;
        }
        unsafe {
            std::ptr::copy_nonoverlapping(
                slice.as_ptr(),
                (self.ptr as usize + (self.length * std::mem::size_of::<u8>())) as *mut u8,
                slice.len(),
            );
            self.length += slice.len();
        }
    }

    fn realloc(&mut self, n: usize) {
        unsafe {
            let lay = Layout::array::<u8>(self.capacity).unwrap();
            let new_size = Layout::array::<u8>(n).unwrap();
            self.ptr = realloc(self.ptr, lay, new_size.size());
        }
    }
}

impl std::default::Default for BytesHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for BytesHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}",self.as_ref())
    }
}

impl std::ops::Drop for BytesHandler {
    fn drop(&mut self) {
        let cap = Layout::array::<u8>(self.capacity).unwrap();
        unsafe {
            if !self.ptr.is_null() {
                dealloc(self.ptr, cap);
            }
        }
    }
}

impl Index<usize> for BytesHandler {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        unsafe {
            if index >= self.length {
                panic!("[BytesHandler] Overflow index");
            }

            &*((self.ptr as usize + (index * std::mem::size_of::<u8>())) as *const u8)
        }
    }
}

impl Index<RangeFrom<usize>> for BytesHandler {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        unsafe {
            if index.start >= self.length {
                panic!("[BytesHandler] Overflow index");
            }
            std::slice::from_raw_parts((self.ptr as usize + (index.start * std::mem::size_of::<u8>())) as *const u8, self.length - index.start)
        }
    }
}

impl Index<RangeTo<usize>> for BytesHandler {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        unsafe {
            if self.ptr.is_null() {
                panic!("[BytesHandler] Overflow index");
            }
            let size = (index.end).min(self.length);
            std::slice::from_raw_parts(self.ptr as *const u8, size)
        }
    }
}

impl Index<RangeToInclusive<usize>> for BytesHandler {
    type Output = [u8];

    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        unsafe {
            if self.ptr.is_null() {
                panic!("[BytesHandler] Overflow index");
            }
            let size = (index.end+1).min(self.length);
            std::slice::from_raw_parts(self.ptr as *const u8, size)
        }
    }
}

impl Index<RangeFull> for BytesHandler {
    type Output = [u8];

    fn index(&self, _index: RangeFull) -> &Self::Output {
        unsafe {
            if self.ptr.is_null() {
                panic!("[BytesHandler] Overflow index");
            }
            std::slice::from_raw_parts(self.ptr as *const u8, self.length)
        }
    }
}

impl IndexMut<usize> for BytesHandler {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.length {
            panic!("[BytesHandler] Overflow index");
        }
        unsafe { &mut *((self.ptr as usize + (index * std::mem::size_of::<u8>())) as *mut u8) }
    }
}

impl IndexMut<RangeFrom<usize>> for BytesHandler {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        if self.length <= (self.length - index.start) {
            panic!("[BytesHandler] Overflow index");
        }
        unsafe { std::slice::from_raw_parts_mut((self.ptr as usize + index.start) as *mut u8, self.length - index.start) }
    }
}

impl IndexMut<RangeTo<usize>> for BytesHandler {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        if self.length <= index.end {
            panic!("[BytesHandler] Overflow index");
        }
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.length.min(index.end)) }
    }
}

impl IndexMut<RangeToInclusive<usize>> for BytesHandler {
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {
        if self.length <= index.end {
            panic!("[BytesHandler] Overflow index");
        }
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.length.min(index.end+1)) }
    }
}

impl IndexMut<RangeFull> for BytesHandler {
    fn index_mut(&mut self, _index: RangeFull) -> &mut Self::Output {
        if self.ptr.is_null() {
            panic!("[BytesHandler] Overflow index");
        }
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.length) }
    }
}

impl AsRef<[u8]> for BytesHandler {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr as *const u8, self.length) }
    }
}

#[cfg(test)]
mod test {
    use crate::io::bytes::BytesHandler;

    #[test]
    fn create_vec() {
        let mut tmp = BytesHandler::new();

        let aux = [2u8; 1000];
        tmp.extend(&aux);
        assert_eq!(aux.len(), tmp.len())
    }

    #[test]
    fn new_vec() {
        let mut tmp = BytesHandler::new();

        let aux = [5u8; 5];
        tmp.extend(&aux);
        assert_eq!([5, 5, 5, 5, 5], tmp.as_ref())
    }

    #[test]
    fn new_vec_multi_ext() {
        let mut tmp = BytesHandler::new();

        let aux = [5u8; 5];
        tmp.extend(&aux);

        let aux = [2u8; 2];
        tmp.extend(&aux);

        let aux = [3u8; 3];
        tmp.extend(&aux);

        let aux = [4u8; 4];
        tmp.extend(&aux);

        assert_eq!([5, 5, 5, 5, 5, 2, 2, 3, 3, 3, 4, 4, 4, 4], tmp.as_ref())
    }

    #[test]
    fn drain() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [0u8; 100];
        tmp.extend(&aux);

        let _aux = tmp.drain(..50).unwrap();
        let tmp = [0u8; 50];
        assert_eq!(tmp, tmp.as_ref())
    }

    #[test]
    fn drain_length() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [0u8; 100];
        tmp.extend(&aux);

        let aux = tmp.drain(..50).unwrap();
        let tmp = [0u8; 50];
        assert_eq!(50, aux.len());
        assert_eq!(50, tmp.len());
    }

    #[test]
    fn drain_cap() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [0u8; 100];
        tmp.extend(&aux);

        let aux = tmp.drain(..50).unwrap();

        assert_eq!(512, aux.cap());
        assert_eq!(512, tmp.cap());
    }

    #[test]
    fn drain_cap_2() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [0u8; 1500];
        tmp.extend(&aux);

        assert_eq!(1536, tmp.cap());
    }

    #[test]
    fn drain_cap_2_512() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [0u8; 512];
        tmp.extend(&aux);
        assert_eq!(512, tmp.cap());
    }

    #[test]
    fn drain_cap_2_513() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [0u8; 1024];
        tmp.extend(&aux);
        
        assert_eq!(1024, tmp.cap());
    }

    #[test]
    fn drain_cap_2_4096() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [0u8; 4096];
        tmp.extend(&aux);
        
        assert_eq!(4096, tmp.cap());
        assert_eq!(aux.len(), tmp.cap());
    }

    #[test]
    fn drain_cap_debug_1() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [0u8; 5];
        tmp.extend(&aux);
        
        assert_eq!(aux, tmp.as_ref());
    }

    #[test]
    fn drain_cap_debug_2() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [0u8; 8196];
        tmp.extend(&aux);
        
        assert_eq!(aux, tmp.as_ref());
    }

    #[test]
    fn drain_cap_ref() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [1,2,3,4,5,6,7,8,9,10,11];
        tmp.extend(&aux);
        
        assert_eq!(aux[2], tmp[2]);
    }

    #[test]
    fn drain_cap_range_to() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [1,2,3,4,5,6,7,8,9,10,11];
        tmp.extend(&aux);
        
        assert_eq!(aux[..3], tmp[..3]);
    }

    #[test]
    fn drain_cap_range_from() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [1,2,3,4,5,6,7,8,9,10,11];
        tmp.extend(&aux);
        assert_eq!(11, tmp.len());
        assert_eq!(aux[2..], tmp[2..]);
    }

    #[test]
    fn drain_cap_range_full() {
        let mut tmp: BytesHandler = BytesHandler::new();

        let aux = [1,2,3,4,5,6,7,8,9,10,11];
        tmp.extend(&aux);
        assert_eq!(11, tmp.len());
        assert_eq!(aux[..], tmp[..]);
    }
}
