#[derive(Clone)]
pub struct BitSet {
    bits: Box<[u32]>,
    len: usize,
}

impl BitSet {
    const BITS: usize = u32::BITS as usize;

    #[inline]
    pub fn new(len: usize) -> Self {
        Self {
            bits: vec![0; len.div_ceil(Self::BITS)].into_boxed_slice(),
            len,
        }
    }

    #[inline(always)]
    pub fn get(&self, i: usize) -> bool {
        assert!(
            i < self.len,
            "index out of bounds: the len is {} but the index is {}",
            self.len,
            i
        );
        let word_idx = i / Self::BITS;
        let bit_idx = i % Self::BITS;

        (self.bits[word_idx] & (1 << bit_idx)) != 0
    }

    #[inline(always)]
    pub fn set(&mut self, i: usize) {
        assert!(
            i < self.len,
            "index out of bounds: the len is {} but the index is {}",
            self.len,
            i
        );
        let word_idx = i / Self::BITS;
        let bit_idx = i % Self::BITS;

        self.bits[word_idx] |= 1 << bit_idx;
    }

    #[inline(always)]
    pub fn reset(&mut self, i: usize) {
        assert!(
            i < self.len,
            "index out of bounds: the len is {} but the index is {}",
            self.len,
            i
        );
        let word_idx = i / Self::BITS;
        let bit_idx = i % Self::BITS;

        self.bits[word_idx] &= !(1 << bit_idx);
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.bits.fill(0);
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = bool> + '_ {
        self.bits
            .iter()
            .copied()
            .flat_map(|word| (0..Self::BITS).map(move |i| (word & (1 << i)) != 0))
            .take(self.len)
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.bits
            .iter()
            .fold(0, |accum, &word| accum + word.count_ones()) as usize
    }

    #[inline]
    pub fn between(&self, start: usize, end: usize) -> u32 {
        let div_start = start / Self::BITS;
        let rem_start = start % Self::BITS;
        let div_end = end / Self::BITS;
        let rem_end = end % Self::BITS;

        if div_start == div_end {
            let mask = !((1 << rem_start) - 1) & ((1 << rem_end) - 1);
            let word = self.bits[div_start];
            let bits = word & mask;

            bits.count_ones()
        } else {
            let start_mask = (u32::MAX >> rem_start) << rem_start;
            let start_word = self.bits[div_start];
            let start_bits = start_word & start_mask;

            let mut bits = start_bits.count_ones();

            for i in div_start + 1..div_end {
                bits += self.bits[i].count_ones();
            }

            let end_mask = (1 << rem_end) - 1;
            let end_word = self.bits[div_end];
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
