use limiter::Limiter;
use sub_images::SubImages;
use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::broadcast::error::TryRecvError;
use tokio::sync::broadcast::Receiver;
use winapi::um::winuser::VK_F1;
use winapi::um::winuser::VK_F10;
use winapi::um::winuser::VK_F11;
use winapi::um::winuser::VK_F12;
use winapi::um::winuser::VK_F2;
use winapi::um::winuser::VK_F3;
use winapi::um::winuser::VK_F4;
use winapi::um::winuser::VK_F5;
use winapi::um::winuser::VK_F6;
use winapi::um::winuser::VK_F7;
use winapi::um::winuser::VK_F8;
use winapi::um::winuser::VK_F9;
use window::AttachedWindow;

mod offsets;
mod sub_images;
mod support;

mod limiter;

fn select_tibia_window() -> anyhow::Result<window::Window> {
   let clients: Vec<window::Window> = window::get_windows().into_iter().filter(|w| w.title.contains("Melimjosias")).collect();

   match clients.len() {
      0 => Err(anyhow::anyhow!("No clients.")),
      1 => Ok(clients.into_iter().next().unwrap()),
      1.. => Ok(clients.into_iter().next().unwrap())
   }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
   let sub_images = sub_images::SubImages::new()?;

   let support_limiter = Limiter::new();
   support_limiter.run(90);

   let key_spammer_limiter = Limiter::new();
   key_spammer_limiter.run(50);

   loop {
      if let Err(err) = client(&sub_images, &support_limiter, &key_spammer_limiter).await {
         println!("CLIENT\t{err}");
      }

      tokio::time::sleep(tokio::time::Duration::from_secs(1)).await
   }
}

async fn client(sub_images: &SubImages, support_limiter: &Limiter, key_spammer_limiter: &Limiter) -> anyhow::Result<()> {
   let tibia_window = select_tibia_window()?.attach();
   let mut screen = image::Image::new(tibia_window.capture_screen());

   loop {
      if !tibia_window.is_alive() {
         return Err(anyhow::anyhow!("Client window was closed."));
      }

      let (sender, receiver) = broadcast::channel::<()>(10);
      let support_receiver = sender.subscribe();

      let support = support::Support::new(&sub_images, support_limiter.clone(), support_receiver, tibia_window.clone(), &mut screen)
         .await
         .map_err(|err| anyhow::anyhow!("Error creating support\t{}", err))?;

      tokio::select! {
         _ = tokio::spawn(key_spammer(key_spammer_limiter.clone(), tibia_window.clone(), receiver)) => (),

         err = support.run() => {
            println!("Support runtime crashed\n{}", err.unwrap_err());
         },
      }

      println!("Restarting client operations");

      _ = sender.send(());
   }
}

async fn key_spammer(limiter: Limiter, tibia_window: AttachedWindow, mut receiver: Receiver<()>) -> anyhow::Result<()> {
   const KEYS: [i32; 12] = [VK_F1, VK_F2, VK_F3, VK_F4, VK_F5, VK_F6, VK_F7, VK_F8, VK_F9, VK_F10, VK_F11, VK_F12];

   loop {
      if tibia_window.is_focused() {
         for key in KEYS {
            if tibia_window.key_state(key) {
               if limiter.allow().await {
                  tibia_window.send_key(key)
               }
            }
         }
      }

      if let Err(TryRecvError::Empty) = receiver.try_recv() {
         continue;
      }

      return Ok(());
   }
}
