#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    tiles: Box<[T]>,
    height: u32,
    width: u32,
}

impl<T: Default + Clone> Grid<T> {
    #[inline]
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            tiles: vec![T::default(); width as usize * height as usize].into_boxed_slice(),
            height,
            width,
        }
    }

    /// Returns the width of the grid.
    #[inline]
    #[must_use]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the grid.
    #[inline]
    #[must_use]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns a `GridBounds` that represents this grid's boundaries.
    #[inline]
    #[must_use]
    pub fn bounds(&self) -> Bounds {
        Bounds::new(self.width, self.height)
    }

    /// Sets the value of the tile at the specified `(x, y)` position within the
    /// grid.
    ///
    /// # Returns whether the provided coordinates were within the grid bounds. That is:
    /// - If the coordinates were within bounds, the associated tile will be set
    ///   to `val`.
    /// - If the coordinates were out of bounds, no changes will be made.
    #[inline]
    pub fn set(&mut self, x: u32, y: u32, val: T) -> bool {
        if let Some(idx) = self.get_index(x, y) {
            self.tiles[idx] = val;
            true
        } else {
            false
        }
    }

    /// Returns a reference to the tile at the specified `(x, y)` position
    /// within the grid.
    ///
    /// Returns `None` if the provided coordinates are not in bounds.
    #[inline]
    #[must_use]
    pub fn get_at(&self, x: u32, y: u32) -> Option<&T> {
        self.get_index(x, y).map(|idx| &self.tiles[idx])
    }

    /// Returns a reference to the tile at the offsetted coordinates provided.
    ///
    /// Returns `None` if the coordinates are not in bounds.
    #[inline]
    #[must_use]
    pub fn get_at_offset(&self, x: u32, y: u32, offset_x: i32, offset_y: i32) -> Option<&T> {
        let bounds = self.bounds();
        bounds
            .get_offset(x, y, offset_x, offset_y)
            .map(|(x, y)| bounds.get_index_unchecked(x, y))
            .map(|idx| &self.tiles[idx])
    }

    /// Returns the provided coordinates translated by the provided offset.
    ///
    /// If the resulting coordinates are out of bounds, this returns `None`.
    #[inline]
    #[must_use]
    pub fn get_offset(&self, x: u32, y: u32, offset_x: i32, offset_y: i32) -> Option<(u32, u32)> {
        self.bounds().get_offset(x, y, offset_x, offset_y)
    }

    /// Returns whether the provided coordinates are within the grid's bounds.
    #[inline]
    #[must_use]
    pub fn within(&self, x: u32, y: u32) -> bool {
        self.bounds().within(x, y)
    }

    /// Returns the index, if any, which the coordinates `(x, y)` specify.
    #[inline]
    #[must_use]
    pub fn get_index(&self, x: u32, y: u32) -> Option<usize> {
        self.bounds().get_index(x, y)
    }

    /// Returns the index, if any, which the coordinates `(x, y)` specify.
    #[inline]
    #[must_use]
    pub fn get_index_unchecked(&self, x: u32, y: u32) -> usize {
        self.bounds().get_index_unchecked(x, y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    height: u32,
    width: u32,
}

impl Bounds {
    #[inline]
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self { height, width }
    }

    /// Returns the width of the grid.
    #[inline]
    #[must_use]
    pub fn width(self) -> u32 {
        self.width
    }

    /// Returns the height of the grid.
    #[inline]
    #[must_use]
    pub fn height(self) -> u32 {
        self.height
    }

    /// Returns the provided coordinates translated by the provided offset.
    ///
    /// If the resulting coordinates are out of bounds, this returns `None`.
    #[inline]
    #[must_use]
    pub fn get_offset(self, x: u32, y: u32, offset_x: i32, offset_y: i32) -> Option<(u32, u32)> {
        let (new_x, new_y) = x
            .checked_add_signed(offset_x)
            .zip(y.checked_add_signed(offset_y))?;
        if self.within(new_x, new_y) {
            Some((new_x, new_y))
        } else {
            None
        }
    }

    /// Returns whether the provided coordinates are within the grid's bounds.
    #[inline]
    #[must_use]
    pub fn within(self, x: u32, y: u32) -> bool {
        x < self.width && y < self.height
    }

    /// Returns the index, if any, which the coordinates `(x, y)` specify.
    #[inline]
    #[must_use]
    pub fn get_index(self, x: u32, y: u32) -> Option<usize> {
        if self.within(x, y) {
            Some(self.get_index_unchecked(x, y))
        } else {
            None
        }
    }

    /// Returns the index, if any, which the coordinates `(x, y)` specify.
    #[inline]
    #[must_use]
    pub fn get_index_unchecked(self, x: u32, y: u32) -> usize {
        y as usize * self.width as usize + x as usize
    }
}
