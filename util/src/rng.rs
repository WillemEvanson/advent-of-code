pub struct Rng(u32);

impl Rng {
    #[inline]
    pub fn new(seed: u32) -> Self {
        Self(seed)
    }

    #[inline]
    pub fn gen_u32(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(747796405).wrapping_add(2891336453);
        let word = ((self.0 >> ((self.0 >> 28) + 4)) ^ self.0).wrapping_mul(277803737);
        (word >> 22) ^ word
    }

    #[inline]
    pub fn gen_bounded_u32(&mut self, range: u32) -> u32 {
        let mut x = self.gen_u32();
        let mut m = x as u64 * range as u64;
        let mut l = m as u32;
        if l < range {
            let mut t = -(range as i32) as u32;
            if t >= range {
                t -= range;
                if t >= range {
                    t %= range;
                }
            }
            while l < t {
                x = self.gen_u32();
                m = x as u64 * range as u64;
                l = m as u32;
            }
        }
        (m >> 32) as u32
    }

    #[inline]
    pub fn gen_f32(&mut self) -> f32 {
        f32::from_bits(127 << 23 | self.gen_u32() >> 9) - 1.0
    }

    #[inline]
    pub fn gen_ranged_f32(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.gen_f32()
    }
}
