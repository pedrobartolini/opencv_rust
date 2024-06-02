use std::sync::Arc;

use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Limiter {
   pub tokens: Arc<Mutex<i64>>
}

impl Limiter {
   pub fn new() -> Self {
      Self { tokens: Arc::new(Mutex::new(0)) }
   }

   pub async fn allow(&self) -> bool {
      let mut tokens = self.tokens.lock().await;

      if *tokens > 10 {
         return false;
      }

      *tokens += 1;

      true
   }

   pub fn run(&self, decrease_rate: u64) -> tokio::task::JoinHandle<()> {
      let tokens = self.tokens.clone();

      tokio::spawn(async move {
         loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(decrease_rate)).await;

            if Arc::strong_count(&tokens) == 1 {
               return;
            }

            let mut tokens = tokens.lock().await;

            if *tokens > 0 {
               *tokens -= 1;
            }
         }
      })
   }
}
