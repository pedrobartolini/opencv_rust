#[derive(Clone)]
pub struct Rect {
   pub rect: opencv::core::Rect
}

impl Rect {
   pub fn new(rect: opencv::core::Rect) -> Self {
      Self { rect }
   }

   pub fn split_horizontally(&self) -> (Rect, Rect) {
      let half_width = self.rect.width / 2;

      let left_side = opencv::core::Rect::new(self.rect.x, self.rect.y, half_width, self.rect.height);
      let right_side = opencv::core::Rect::new(self.rect.x + half_width, self.rect.y, half_width, self.rect.height);

      (Rect::new(left_side), Rect::new(right_side))
   }

   pub fn split_vertically(&self) -> (Rect, Rect) {
      let half_height = self.rect.height / 2;

      let top_side = opencv::core::Rect::new(self.rect.x, self.rect.y, self.rect.width, half_height);
      let bottom_side = opencv::core::Rect::new(self.rect.x, self.rect.y + half_height, self.rect.width, half_height);

      (Rect::new(top_side), Rect::new(bottom_side))
   }
}
