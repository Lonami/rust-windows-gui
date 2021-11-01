use winapi::shared::windef::RECT;

#[derive(Clone)]
pub struct Rect(pub(crate) RECT);

impl Rect {
    /// Create a new rect with the given dimensions.
    pub fn new(width: i32, height: i32) -> Self {
        Self(RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        })
    }

    /// Construct a new rect with the same size but at the new location.
    pub fn at(&self, x: i32, y: i32) -> Self {
        Self(RECT {
            left: x,
            top: y,
            right: x + self.width(),
            bottom: y + self.height(),
        })
    }

    /// Construct a new rect at the same position but with a new size.
    ///
    /// This will cause the sides to switch positions when negative values are used.
    pub fn sized(&self, width: i32, height: i32) -> Self {
        Self(RECT {
            left: self.0.left + width.min(0),
            top: self.0.top + height.min(0),
            right: self.0.left + width.max(0),
            bottom: self.0.top + height.max(0),
        })
    }

    /// Construct a new rect at the same position but extending or stretching its size.
    ///
    /// This will cause the sides to switch positions when the deltas' absolute values are larger
    /// than the current dimension and their sign is negative.
    pub fn resized_by(&self, delta_width: i32, delta_height: i32) -> Self {
        self.sized(self.width() + delta_width, self.height() + delta_height)
    }

    pub fn left(&self) -> i32 {
        self.0.left
    }

    pub fn top(&self) -> i32 {
        self.0.top
    }

    pub fn right(&self) -> i32 {
        self.0.right
    }

    pub fn bottom(&self) -> i32 {
        self.0.bottom
    }

    /// Alias for [`Self::left`].
    pub fn x(&self) -> i32 {
        self.0.left
    }

    /// Alias for [`Self::top`].
    pub fn y(&self) -> i32 {
        self.0.top
    }

    /// Calculates the difference between the right and the left margins.
    pub fn width(&self) -> i32 {
        self.0.right - self.0.left
    }

    /// Calculates the difference between the bottom and the top margins.
    pub fn height(&self) -> i32 {
        self.0.bottom - self.0.top
    }
}
