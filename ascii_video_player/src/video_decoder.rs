use anyhow::Result;
use ffmpeg_next::{
    codec, decoder,
    format::{input, Pixel},
    frame, media, software,
    util::frame::video::Video,
};
use std::path::Path;

pub struct VideoDecoder {
    decoder: decoder::Video,
    scaler: software::scaling::Context,
    input_context: input::Context,
    frame: frame::Video,
    scaled_frame: frame::Video,
    frame_rate: f64,
}

impl VideoDecoder {
    pub fn new(path: &Path) -> Result<Self> {
        let input_context = input::open(path)?;

        let input = input_context
            .streams()
            .best(media::Type::Video)
            .ok_or_else(|| anyhow::anyhow!("No video stream found"))?;
        let context = codec::Context::from_parameters(input.parameters())?;
        let decoder = context.decoder().video()?;

        let frame_rate = f64::from(input.rate().0) / f64::from(input.rate().1);

        let mut scaler = software::scaling::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::GRAY8,
            decoder.width(),
            decoder.height(),
            software::scaling::Flags::BILINEAR,
        )?;

        let frame = frame::Video::empty();
        let scaled_frame = frame::Video::empty();

        Ok(Self {
            decoder,
            scaler,
            input_context,
            frame,
            scaled_frame,
            frame_rate,
        })
    }

    pub fn get_fps(&self) -> u32 {
        self.frame_rate as u32
    }

    pub fn next_frame(&mut self) -> Result<Option<Vec<u8>>> {
        let mut result = None;

        for (stream, packet) in self.input_context.packets() {
            if stream.index() == 0 {
                self.decoder.send_packet(&packet)?;
                if self.decoder.receive_frame(&mut self.frame).is_ok() {
                    self.scaler.run(&self.frame, &mut self.scaled_frame)?;
                    let data = self.scaled_frame.data(0).to_vec();
                    result = Some(data);
                    break;
                }
            }
        }

        Ok(result)
    }
}
