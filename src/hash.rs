pub mod hash {
    pub trait Hashable {
        fn hash(&self) -> usize;
    }

    impl Hashable for String {
        // https://theartincode.stanis.me/008-djb2/
        fn hash(&self) -> usize {
            let mut result: usize = 5381;
            for c in self.bytes() {
                result =
                    (result << 5).wrapping_add(result).wrapping_add(c as usize);
            }
            result
        }
    }
}
