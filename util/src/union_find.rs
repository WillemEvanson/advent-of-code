#[derive(Debug, Clone)]
pub struct UnionFind {
    parents: Box<[u32]>,
    sizes: Box<[u32]>,
}

impl UnionFind {
    #[inline]
    pub fn new(n: u32) -> Self {
        Self {
            parents: (0..n).collect::<Box<[_]>>(),
            sizes: (0..n).map(|_| 1).collect::<Box<[_]>>(),
        }
    }

    #[inline]
    pub fn find_const(&self, mut i: u32) -> u32 {
        while self.parents[i as usize] != i {
            i = self.parents[i as usize]
        }
        i
    }

    #[inline]
    pub fn find(&mut self, mut i: u32) -> u32 {
        let mut root = i;
        while self.parents[root as usize] != root {
            root = self.parents[root as usize];
        }

        while self.parents[i as usize] != i {
            let next = self.parents[i as usize];
            self.parents[i as usize] = root;
            i = next;
        }

        root
    }

    #[inline]
    pub fn merge(&mut self, i: u32, j: u32) {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.sizes[root_i as usize] < self.sizes[root_j as usize] {
                self.sizes[root_i as usize] += self.sizes[root_j as usize];
                self.parents[root_i as usize] = root_j;
            } else {
                self.sizes[root_j as usize] += self.sizes[root_i as usize];
                self.parents[root_j as usize] = root_i;
            }
        }
    }
}
