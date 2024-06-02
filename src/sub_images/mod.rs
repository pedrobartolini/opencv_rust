use opencv::imgcodecs::imdecode;
use opencv::imgcodecs::IMREAD_COLOR;

pub const HEALTH_MANA_BAR: &'static [u8] = include_bytes!("health_mana_bar.png");
pub const ANY_BAR: &'static [u8] = include_bytes!("any_bar.png");
pub const BATTLE_BAR_WITH_STOP: &'static [u8] = include_bytes!("battle_bar_with_stop.png");
pub const BATTLE_BAR: &'static [u8] = include_bytes!("battle_bar.png");
pub const PZ_ICON: &'static [u8] = include_bytes!("pz_icon.png");
pub const HUNGER_ICON: &'static [u8] = include_bytes!("hunger_icon.png");
pub const HASTE_ICON: &'static [u8] = include_bytes!("haste_icon.png");

fn decode(buf: &[u8]) -> anyhow::Result<image::Image> {
   Ok(image::Image::new(imdecode(&buf, IMREAD_COLOR)?))
}

pub struct SubImages {
   pub health_mana_bar:      image::Image,
   pub any_bar:              image::Image,
   pub battle_bar_with_stop: image::Image,
   pub battle_bar:           image::Image,
   pub pz_icon:              image::Image,
   pub hunger_icon:          image::Image,
   pub haste_icon:           image::Image
}

impl SubImages {
   pub fn new() -> anyhow::Result<Self> {
      let health_mana_bar = decode(&HEALTH_MANA_BAR)?;
      let any_bar = decode(&ANY_BAR)?;
      let battle_bar_with_stop = decode(&BATTLE_BAR_WITH_STOP)?;
      let battle_bar = decode(&BATTLE_BAR)?;
      let pz_icon = decode(&PZ_ICON)?;
      let hunger_icon = decode(&HUNGER_ICON)?;
      let haste_icon = decode(&HASTE_ICON)?;

      Ok(Self {
         health_mana_bar,
         any_bar,
         battle_bar_with_stop,
         battle_bar,
         pz_icon,
         hunger_icon,
         haste_icon
      })
   }
}
