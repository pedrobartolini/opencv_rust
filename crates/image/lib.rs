use opencv::core::min_max_loc;
use opencv::core::Mat;
use opencv::core::MatTraitConst;
use opencv::core::Point;
use opencv::core::Rect as OpencvRect;
use opencv::core::Scalar;
use opencv::core::Size;
use opencv::core::Vec3b;
use opencv::highgui::destroy_window;
use opencv::highgui::imshow;
use opencv::highgui::wait_key;
use opencv::imgproc::cvt_color;
use opencv::imgproc::match_template;
use opencv::imgproc::rectangle;
use opencv::imgproc::COLOR_BGR2GRAY;
use opencv::imgproc::TM_CCOEFF_NORMED;

mod rect;

pub use rect::Rect;

#[derive(Clone)]
pub struct Image {
   pub mat: Mat
}

unsafe impl Send for Image {}

impl Image {
   pub fn new(mat: Mat) -> Self {
      Self { mat }
   }

   pub fn height(&self) -> i32 {
      self.mat.rows()
   }

   pub fn width(&self) -> i32 {
      self.mat.cols()
   }

   pub fn show(&self) -> anyhow::Result<()> {
      let uid = nanoid::nanoid!();

      imshow(&uid, &self.mat)?;
      wait_key(0)?;
      _ = destroy_window(&uid);

      Ok(())
   }

   pub fn crop(&self, rect: &rect::Rect) -> anyhow::Result<Image> {
      if rect.rect.x < 0 || rect.rect.y < 0 || (rect.rect.x + rect.rect.width) > self.width() || rect.rect.y + rect.rect.height > self.height() {
         return Err(anyhow::anyhow!("Rect is out of bounds of image"));
      }

      Ok(Image::new(Mat::roi(&self.mat, rect.rect)?.clone_pointee()))
   }

   pub fn contain(&self, target: &Image, threshold: f64) -> anyhow::Result<bool> {
      if self.height() < target.height() || self.width() < target.width() {
         return Err(anyhow::anyhow!("Target is bigger than source"));
      }

      let mut result = Mat::default();
      match_template(&self.mat, &target.mat, &mut result, TM_CCOEFF_NORMED, &opencv::core::no_array())?;

      let mut max_val: f64 = 0.0;
      min_max_loc(&result, None, Some(&mut max_val), None, None, &opencv::core::no_array())?;

      Ok(max_val >= threshold)
   }

   pub fn find_rect(&self, target: &Image, threshold: f32) -> anyhow::Result<rect::Rect> {
      if self.height() < target.height() || self.width() < target.width() {
         return Err(anyhow::anyhow!("Target is bigger than source"));
      }

      let mut gray = Mat::default();
      cvt_color(&self.mat, &mut gray, COLOR_BGR2GRAY, 0)?;

      let mut target_gray = Mat::default();
      cvt_color(&target.mat, &mut target_gray, COLOR_BGR2GRAY, 0)?;

      let mut result = Mat::default();
      match_template(&gray, &target_gray, &mut result, TM_CCOEFF_NORMED, &opencv::core::no_array())?;

      let mut point = Point::default();
      min_max_loc(&result, None, None, None, Some(&mut point), &opencv::core::no_array())?;

      let rect = OpencvRect::from_point_size(
         point,
         Size {
            width:  target_gray.cols(),
            height: target_gray.rows()
         }
      );

      if *result.at_2d::<f32>(point.y, point.x)? < threshold {
         return Err(anyhow::anyhow!("Threshold was not achieved"));
      }

      return Ok(rect::Rect { rect });
   }

   pub fn draw(&mut self, rect: &rect::Rect) -> anyhow::Result<()> {
      rectangle(&mut self.mat, rect.rect, Scalar::new(0.0, 0.0, 255.0, 0.0), 2, 8, 0)?;
      Ok(())
   }

   pub fn pixel<'a>(&'a self, x: i32, y: i32) -> anyhow::Result<&'a Vec3b> {
      Ok(self.mat.at_2d::<Vec3b>(y, x)?)
   }

   pub fn is_red(&self, x: i32, y: i32) -> anyhow::Result<bool> {
      Ok(self.pixel(x, y)?.get(2).is_some_and(|red| *red > 180))
   }

   pub fn is_blue(&self, x: i32, y: i32) -> anyhow::Result<bool> {
      Ok(self.pixel(x, y)?.get(0).is_some_and(|blue| *blue > 180))
   }
}
