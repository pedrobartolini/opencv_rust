use super::*;

impl<'a> Support<'a> {
   pub fn health_percentage(&self) -> anyhow::Result<u8> {
      let mut health_end = self.health_end;

      loop {
         if health_end <= self.health_rect.rect.x {
            break Ok(0);
         }

         if self.screen.is_red(health_end, self.health_rect.rect.y)? {
            break Ok(((health_end - self.health_rect.rect.x) * 100 / self.health_rect.rect.width) as u8);
         }

         health_end -= 1;
      }
   }

   pub fn mana_percentage(&self) -> anyhow::Result<u8> {
      let mut mana_end = self.mana_end;

      loop {
         if mana_end <= self.mana_rect.rect.x {
            break Ok(0);
         }

         if self.screen.is_blue(mana_end, self.mana_rect.rect.y)? {
            break Ok(((mana_end - self.mana_rect.rect.x) * 100 / self.mana_rect.rect.width) as u8);
         }

         mana_end -= 1;
      }
   }
}
