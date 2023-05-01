use rodio::{OutputStream, OutputStreamHandle, Sink};

pub trait Sfx<TSource> {
    fn open(self) -> TSource;
}

pub trait Music<TSource>: Sfx<TSource> {
    fn id(&self) -> u32;
}

pub struct Sound2 {
    handle: OutputStreamHandle,
    music: Option<(u32, Sink)>,
    sfx: Option<Sink>,
    _stream: OutputStream,
}

impl Sound2 {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();

        Sound2 {
            _stream: stream,
            music: None,
            handle,
            sfx: None,
        }
    }

    pub fn stop_music(&mut self) {
        if let Some((_, sink)) = self.music.take() {
            sink.stop();
        }
    }

    fn is_playing_music(&self, id: u32) -> bool {
        if let Some((playing, _)) = self.music.as_ref() {
            *playing == id
        } else {
            false
        }
    }

    pub fn start_music<T, TSource>(&mut self, music: T)
    where
        T: Music<TSource>,
        TSource: rodio::Source + Send + 'static,
        f32: cpal::FromSample<TSource::Item>,
        TSource::Item: rodio::Sample + Send,
    {
        let id = music.id();

        if self.is_playing_music(id) {
            return; // Allready playing this music
        }

        self.stop_music();

        let sink = Sink::try_new(&self.handle).unwrap();
        sink.append(music.open());
        self.music = Some((id, sink));
    }

    pub fn play_sfx<T, TSource>(&mut self, sound: T)
    where
        T: Sfx<TSource>,
        TSource: rodio::Source + Send + 'static,
        f32: cpal::FromSample<TSource::Item>,
        TSource::Item: rodio::Sample + Send,
    {
        if let Some(sink) = self.sfx.take() {
            sink.stop();
        }

        let sink = Sink::try_new(&self.handle).unwrap();
        sink.append(sound.open());
        self.sfx = Some(sink);
    }
}
