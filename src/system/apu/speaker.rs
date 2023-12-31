use flume::{Receiver, Sender};
use anyhow::{anyhow, Context, Result};
use cpal::{Device, FromSample, OutputCallbackInfo, SampleFormat, SizedSample, Stream, SupportedStreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crate::codeloc;

pub struct Speaker
{
    audio_stream : Stream,
    sample_rate : u32,
    current_sample_index : u32,
    sample_value_sender : Sender<f64>,
}

impl Speaker
{
    pub fn new() -> Result<Speaker>
    {
        let device = cpal::default_host().default_output_device().context(codeloc!())?;
        let config = device.default_output_config().context(codeloc!())?;
        let (waveform_sender, waveform_receiver) = flume::unbounded::<f64>();

        let audio_stream = match config.sample_format()
        {
            SampleFormat::I8  => Speaker::create_audio_stream::<i8 >(&device, &config, waveform_receiver),
            SampleFormat::I16 => Speaker::create_audio_stream::<i16>(&device, &config, waveform_receiver),
            SampleFormat::I32 => Speaker::create_audio_stream::<i32>(&device, &config, waveform_receiver),
            SampleFormat::I64 => Speaker::create_audio_stream::<i64>(&device, &config, waveform_receiver),
            SampleFormat::U8  => Speaker::create_audio_stream::<u8 >(&device, &config, waveform_receiver),
            SampleFormat::U16 => Speaker::create_audio_stream::<u16>(&device, &config, waveform_receiver),
            SampleFormat::U32 => Speaker::create_audio_stream::<u32>(&device, &config, waveform_receiver),
            SampleFormat::U64 => Speaker::create_audio_stream::<u64>(&device, &config, waveform_receiver),
            SampleFormat::F32 => Speaker::create_audio_stream::<f32>(&device, &config, waveform_receiver),
            SampleFormat::F64 => Speaker::create_audio_stream::<f64>(&device, &config, waveform_receiver),
            _ => Err(anyhow!("Unsupported sample format"))
        }.context(codeloc!())?;

        return Ok(Speaker
        {
            audio_stream: audio_stream,
            sample_rate: config.sample_rate().0,
            current_sample_index: 0,
            sample_value_sender: waveform_sender,
        });
    }

    fn create_audio_stream<T>(device : &Device, config : &SupportedStreamConfig, waveform_receiver : Receiver<f64>)
        -> Result<Stream> where T : SizedSample, T : FromSample<f64>
    {
        let number_of_channels = config.channels() as usize;
        let error_callback = |err| eprintln!("Error building output sound stream: {}", err);

        let data_callback = move |output : &mut [T], _ : &OutputCallbackInfo|
        {
            for frame in output.chunks_mut(number_of_channels)
            {
                let value = T::from_sample(waveform_receiver.recv().unwrap_or_default());
                for frame_value in frame.iter_mut() { *frame_value = value; }
            }
        };

        return Ok(device.build_output_stream(&config.config(), data_callback, error_callback, None).context(codeloc!())?);
    }

    pub fn play(&self) { self.audio_stream.play().unwrap_or_default() }
    pub fn pause(&self) { self.audio_stream.pause().unwrap_or_default() }

    pub fn advance_to_next_waveform_index(self : &mut Speaker) -> f64
    {
        self.current_sample_index = (self.current_sample_index+1) % self.sample_rate;
        return (self.current_sample_index as f64) / (self.sample_rate as f64);
    }

    pub fn accept_waveform_value(&self, value : f64)
    {
        self.sample_value_sender.send(value).unwrap_or_default();
    }
}
