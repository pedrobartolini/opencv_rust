use crate::sub_images::SubImages;

pub fn get_health_and_mana_offsets(screen: &mut image::Image, sub_images: &SubImages) -> anyhow::Result<(image::Rect, image::Rect)> {
   let (top_rect, bottom_rect) = screen.find_rect(&sub_images.health_mana_bar, 0.5)?.split_vertically();

   let relative_health_rect = screen.crop(&top_rect)?.find_rect(&sub_images.any_bar, 0.5)?;
   let health_rect = image::Rect::new(relative_health_rect.rect + top_rect.rect.tl());
   let health_rect = image::Rect::new(opencv::core::Rect::new(
      health_rect.rect.x,
      health_rect.rect.y + health_rect.rect.height / 2,
      health_rect.rect.width - 2,
      1
   ));

   let relative_mana_rect = screen.crop(&bottom_rect)?.find_rect(&sub_images.any_bar, 0.5)?;
   let mana_rect = image::Rect::new(relative_mana_rect.rect + bottom_rect.rect.tl());
   let mana_rect = image::Rect::new(opencv::core::Rect::new(mana_rect.rect.x, mana_rect.rect.y + mana_rect.rect.height / 2, mana_rect.rect.width - 2, 1));

   screen.draw(&health_rect)?;
   screen.draw(&mana_rect)?;

   Ok((health_rect, mana_rect))
}

pub fn get_battle_bar_offsts(screen: &mut image::Image, sub_images: &SubImages) -> anyhow::Result<image::Rect> {
   let battle_bar_with_stop_rect = screen.find_rect(&sub_images.battle_bar_with_stop, 0.5)?;

   let relative_battle_bar_rect = screen.crop(&battle_bar_with_stop_rect)?.find_rect(&sub_images.battle_bar, 0.05)?;

   let battle_bar_rect = image::Rect::new(relative_battle_bar_rect.rect + battle_bar_with_stop_rect.rect.tl());

   screen.draw(&battle_bar_rect)?;

   Ok(battle_bar_rect)
}
