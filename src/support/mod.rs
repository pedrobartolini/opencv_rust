use winapi::um::winuser::VK_END;
use winapi::um::winuser::VK_F6;
use winapi::um::winuser::VK_HOME;
use winapi::um::winuser::VK_SCROLL;

use super::*;
use crate::limiter::Limiter;

mod healer;

pub struct Support<'a> {
   limiter:  Limiter,
   receiver: Receiver<()>,

   tibia_window: window::AttachedWindow,
   screen:       &'a mut image::Image,
   sub_images:   &'a sub_images::SubImages,

   count:   u64,
   started: std::time::Instant,

   battle_rect: image::Rect,
   health_rect: image::Rect,
   mana_rect:   image::Rect,

   health_end: i32,
   mana_end:   i32
}

impl<'a> Support<'a> {
   pub async fn new(sub_images: &'a sub_images::SubImages, limiter: Limiter, receiver: Receiver<()>, tibia_window: window::AttachedWindow, screen: &'a mut image::Image) -> anyhow::Result<Self> {
      let mut screen_tmp = image::Image::new(tibia_window.capture_screen());

      let (health_rect, mana_rect) = offsets::get_health_and_mana_offsets(&mut screen_tmp, &sub_images).map_err(|err| anyhow::anyhow!("Can't find hp/mp rect. {}", err))?;
      let health_end = health_rect.rect.x + health_rect.rect.width;
      let mana_end = mana_rect.rect.x + mana_rect.rect.width;
      let battle_rect = offsets::get_battle_bar_offsts(&mut screen_tmp, &sub_images).map_err(|err| anyhow::anyhow!("Can't find battle rect. {}", err))?;

      // screen_tmp.show()?;

      Ok(Self {
         receiver,
         limiter,
         tibia_window,
         screen,
         count: 50,
         started: std::time::Instant::now(),
         sub_images,
         battle_rect,
         health_rect,
         mana_rect,
         health_end,
         mana_end
      })
   }

   pub async fn run(mut self) -> anyhow::Result<()> {
      let name = {
         let tmp = self.tibia_window.title.trim_start_matches("Tibia - ");
         tmp.split(" - ").next().unwrap_or(&tmp).to_string()
      };

      self.started = std::time::Instant::now();

      loop {
         *self.screen = image::Image::new(self.tibia_window.capture_screen());

         let (hp_percentage, mp_percentage, is_pz, is_hasted, is_hungry) = self.single_iteration().await?;

         if self.count % 50 == 0 {
            _ = clearscreen::clear();

            let started_secs = self.started.elapsed().as_secs();
            if started_secs > 1 {
               println!("TKS\t{}\n", self.count / started_secs);
            }

            println!("NAME\t{}", name);
            println!("HEALTH\t{}%", hp_percentage);
            println!("MANA\t{}%", mp_percentage);

            let mut statuses = vec![];

            if is_pz {
               statuses.push("PROTECTION_ZONE");
            }

            if is_hasted {
               statuses.push("HASTED")
            }

            if is_hungry {
               statuses.push("HUNGRY")
            }

            println!("STATUS\t{:?}", statuses);

            if self.count % 5000 == 0 {
               self.started = std::time::Instant::now();
               self.count = 0;
            }
         }

         self.count += 1;

         if let Err(TryRecvError::Empty) = self.receiver.try_recv() {
            continue;
         }

         break;
      }

      Ok(())
   }

   pub async fn single_iteration(&mut self) -> anyhow::Result<(u8, u8, bool, bool, bool)> {
      const MIN_MANA: u8 = 45;

      let health_percentage = self.health_percentage()?;
      let mana_percentage = self.mana_percentage()?;

      let battle = self.screen.crop(&self.battle_rect)?;
      let is_pz = battle.contain(&self.sub_images.pz_icon, 0.8)?;
      let is_hasted = battle.contain(&self.sub_images.haste_icon, 0.8)?;
      let is_hungry = battle.contain(&self.sub_images.hunger_icon, 0.8)?;

      if health_percentage < 35 {
         if self.limiter.allow().await {
            self.tibia_window.send_key(VK_F5);
         }
      }

      if health_percentage < 50 {
         if self.limiter.allow().await {
            self.tibia_window.send_key(VK_F1);
         }
      }

      if health_percentage < 75 && mana_percentage > MIN_MANA {
         if self.limiter.allow().await {
            self.tibia_window.send_key(VK_F12);
         }
      }

      if mana_percentage <= MIN_MANA {
         if self.limiter.allow().await {
            self.tibia_window.send_key(VK_F2);
         }
      }

      if !is_pz && !is_hasted && mana_percentage > MIN_MANA {
         if self.limiter.allow().await {
            self.tibia_window.send_key(VK_F6);
         }
      }

      if is_hungry {
         if self.limiter.allow().await {
            self.tibia_window.send_key(VK_F10);
         }
      }

      if mana_percentage >= 100 {
         if self.limiter.allow().await {
            self.tibia_window.send_key(VK_F12);
         }
      }

      return Ok((health_percentage, mana_percentage, is_pz, is_hasted, is_hungry));
   }
}
