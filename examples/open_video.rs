use video_rs::{
  self,
  Locator,
  Decoder,
};

use std::path::PathBuf;

fn main() {
  video_rs::init();

  let source: Locator = PathBuf::from("./data/VID_20221111_144551.mp4").into();
  let mut decoder = Decoder::new(&source)
    .expect("failed to create decoder");

  for (i, frame) in decoder.decode_iter().enumerate() {
    if let Ok((_, frame)) = frame {
      let rgb = frame
        .slice(ndarray::s![0, 0, ..])
        .to_slice()
        .unwrap();
      println!(
        "{i}) pixel at 0, 0: {}, {}, {}",
        rgb[0], rgb[1], rgb[2],
      );
    } else {
      break;
    }
  }

}
