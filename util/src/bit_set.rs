#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BitSet {
    bits: Box<[u32]>,
    len: u32,
}

impl BitSet {
    const BITS: u32 = u32::BITS;

    #[inline]
    pub fn new(len: u32) -> Self {
        Self {
            bits: vec![0; len.div_ceil(Self::BITS) as usize].into_boxed_slice(),
            len,
        }
    }

    // Return whether the bit at position `i` is set.
    #[inline(always)]
    pub fn get(&self, i: u32) -> bool {
        assert!(
            i < self.len,
            "index out of bounds: the len is {} but the index is {}",
            self.len,
            i
        );
        let word_idx = i / Self::BITS;
        let bit_idx = i % Self::BITS;

        (self.bits[word_idx as usize] & (1 << bit_idx)) != 0
    }

    // Sets the bit at position `i`.
    #[inline(always)]
    pub fn set(&mut self, i: u32) {
        assert!(
            i < self.len,
            "index out of bounds: the len is {} but the index is {}",
            self.len,
            i
        );
        let word_idx = i / Self::BITS;
        let bit_idx = i % Self::BITS;

        self.bits[word_idx as usize] |= 1 << bit_idx;
    }

    // Unsets the bit at position `i`.
    #[inline(always)]
    pub fn unset(&mut self, i: u32) {
        assert!(
            i < self.len,
            "index out of bounds: the len is {} but the index is {}",
            self.len,
            i
        );
        let word_idx = i / Self::BITS;
        let bit_idx = i % Self::BITS;

        self.bits[word_idx as usize] &= !(1 << bit_idx);
    }

    // Unsets every bit in the set.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.bits.fill(0);
    }

    // Returns an iterator over the status of the individual bits.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = bool> + '_ {
        self.bits
            .iter()
            .copied()
            .flat_map(|word| (0..Self::BITS).map(move |i| (word & (1 << i)) != 0))
            .take(self.len as usize)
    }

    // Returns the number of set bits.
    #[inline]
    pub fn count(&self) -> u32 {
        self.bits
            .iter()
            .fold(0, |accum, &word| accum + word.count_ones())
    }

    // Returns the number of set bits in the range `start`..=`end`,
    #[inline]
    pub fn between(&self, start: u32, end: u32) -> u32 {
        let div_start = start / Self::BITS;
        let rem_start = start % Self::BITS;
        let div_end = end / Self::BITS;
        let rem_end = end % Self::BITS;

        if div_start == div_end {
            let mask = !((1 << rem_start) - 1) & ((1 << rem_end) - 1);
            let word = self.bits[div_start as usize];
            let bits = word & mask;

            bits.count_ones()
        } else {
            let start_mask = (u32::MAX >> rem_start) << rem_start;
            let start_word = self.bits[div_start as usize];
            let start_bits = start_word & start_mask;
            let mut bits = start_bits.count_ones();

            for i in div_start + 1..div_end {
                bits += self.bits[i as usize].count_ones();
            }

            let end_mask = (1 << rem_end) - 1;
            let end_word = self.bits[div_end as usize];
            let end_bits = end_word & end_mask;
            bits += end_bits.count_ones();

            bits
        }
    }
}

impl std::fmt::Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for elem in self.iter() {
            if elem {
                f.write_str("1")?;
            } else {
                f.write_str("0")?;
            }
        }
        Ok(())
    }
}
