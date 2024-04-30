pub mod sounds
{
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, Sink};
    use std::path::{Path,PathBuf};

    pub struct Sound
    {
        _stream: rodio::OutputStream,
        _stream_handle: rodio::OutputStreamHandle,
        sink: rodio::Sink,
        elapsed: u32,
        start_time: std::time::Instant,
        paused: bool,
        duration: u32
    }

    impl Sound
    {
        pub fn new() -> Sound
        {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            Sound
            {
                _stream,
                _stream_handle: stream_handle,
                sink,
                elapsed: 0,
                start_time: std::time::Instant::now(),
                paused: false,
                duration: 0
            }
        }
        pub fn open(&mut self, path: &Path, skip: u32) -> Result<(), Box<dyn std::error::Error>>
        {
            use rodio::Source;
            use std::time::*;
            use lofty::read_from_path;
            use lofty::prelude::AudioFile;

            fn get_duration(p: PathBuf) -> Result<u32,Box<dyn std::error::Error>>
            {
                let audio = read_from_path(p)?;
                let prop = audio.properties();
                Ok(prop.duration().as_millis() as u32)
            }

            let file = File::open(path)?;
            let f = BufReader::new(file);
            let source = Decoder::new(f)?;
            let s = source.skip_duration(Duration::from_millis(skip as u64));
            self.elapsed=skip;
            self.sink.append(s);
            self.duration = get_duration(path.to_path_buf()).ok().unwrap();
            Ok(())
        }
        pub fn action(&mut self, a: i32)
        {
            match a
            {
                0 => {
                    self.sink.play();
                    self.start_time = std::time::Instant::now();
                    self.paused=false
                },
                1 => {
                    self.sink.pause();
                    self.elapsed += self.start_time.elapsed().as_millis() as u32;
                    self.paused=true;
                },
                2 => {
                    self.sink.stop();
                },
                _ => ()
            }
        }
        pub fn ended(&self) -> bool
        {
            self.sink.empty()
        }
        pub fn elapsed(&self) -> u32
        {
            if self.paused
            {
                return self.elapsed;
            }
            self.elapsed + self.start_time.elapsed().as_millis() as u32
        }
    }

    pub struct Player
    {
        sound: Sound,
        queue: Vec<PathBuf>,
        qinfo: (usize,usize,bool), //head, len, started
        current: String,
        current_path: PathBuf,
        state: i32,
        repeat: bool
    }

    impl Player
    {
        pub fn new() -> Player
        {
            Player
            {
                sound: Sound::new(),
                queue: Vec::new(),
                qinfo: (0,0,false),
                current: "".to_string(),
                current_path: "".into(),
                state: 0,
                repeat: false
            }
        }

        pub fn prev(&mut self)
        {
            if self.queue.is_empty()
            {
                return;
            }
            if self.qinfo.0.wrapping_sub(1) == usize::MAX
            {
                if !self.repeat
                {
                    return;
                }
                else
                {
                    self.qinfo.0 = self.qinfo.1;
                }
            }
            self.qinfo.0 = (self.qinfo.0).wrapping_sub(1);

            self.sound.action(2);
            self.sound = Sound::new();
            let mut ok = false;
            let start = self.current_path.clone();
            while !ok
            {
                if self.queue[self.qinfo.0].to_str().unwrap_or_default() == start.to_str().unwrap_or_default()
                {
                    self.reset();
                    return;
                }
                match self.sound.open(&self.queue[self.qinfo.0],0)
                {
                    Ok(_) => {
                        ok = true;
                        if self.state==1
                        {
                            self.sound.action(1);
                        }
                        self.current = self.queue[self.qinfo.0].file_stem().unwrap().to_str().unwrap().to_string();
                        self.current_path = self.queue[self.qinfo.0].clone();
                    },
                    Err(_) => {
                        self.qinfo.0-=1;
                        if self.qinfo.0 == 0
                        {
                            self.qinfo.0+=1;
                            if !self.repeat
                            {
                                return;
                            }
                            self.qinfo.0 = self.qinfo.1-1;
                        }
                    }
                }
            }
        }
        pub fn skip(&mut self)
        {
            if self.queue.is_empty()
            {
                return;
            }
            if self.qinfo.0+1 == self.qinfo.1
            {
                if !self.repeat
                {
                    return;
                }
                else
                {
                    self.qinfo.0 = usize::MAX;
                }
            }
            if self.qinfo.2
            {
                self.qinfo.0 = (self.qinfo.0).wrapping_add(1);
            }
            else
            {
                self.qinfo.0 = 0;
            }
            self.sound.action(2);
            self.sound = Sound::new();

            let mut ok = false;
            let start = self.current_path.clone();
            while !ok
            {
                if self.queue[self.qinfo.0].to_str().unwrap_or_default() == start.to_str().unwrap_or_default()
                {
                    self.reset();
                    return;
                }
                match self.sound.open(&self.queue[self.qinfo.0],0)
                {
                    Ok(_) => {
                        ok = true;
                        self.qinfo.2 = true;
                        if self.state==1
                        {
                            self.sound.action(1);
                        }
                        self.current = self.queue[self.qinfo.0].file_stem().unwrap().to_str().unwrap().to_string();
                        self.current_path = self.queue[self.qinfo.0].clone();
                    },
                    Err(_) => {
                        self.qinfo.0+=1;
                        if self.qinfo.0 >= self.qinfo.1
                        {
                            self.qinfo.0-=1;
                            if !self.repeat
                            {
                                return;
                            }
                            self.qinfo.0 = 0;
                        }
                    }
                }
            }
        }
        pub fn get_repeat(&self) -> bool
        {
            self.repeat
        }

        pub fn toggle_repeat(&mut self)
        {
            self.repeat = !self.repeat;
        }
        pub fn seek(&mut self, skip: u32)
        {
            self.sound.action(2);
            self.sound = Sound::new();
            match self.sound.open(&self.queue[self.qinfo.0],skip)
            {
                Ok(_) => (),
                Err(_) => {self.skip()}
            }
            if self.state == 1
            {
                self.sound.action(1);
            }
        }
        pub fn add(&mut self, p: PathBuf)
        {
            self.queue.push(p);
            self.qinfo.1 += 1;
        }
        pub fn play_pause(&mut self)
        {
            let newstate = (self.state+1)%2;
            self.sound.action(newstate);
            self.state=newstate;
        }
        pub fn get_duration(&self) -> u32
        {
            self.sound.duration
        }
        pub fn update(&mut self)
        {
            if self.sound.ended() && !self.queue.is_empty()
            {
                self.skip()
            }
        }
        pub fn reset(&mut self)
        {
            self.sound.action(2);
            self.queue = Vec::new();
            self.qinfo = (0,0,false);
            self.current="".to_string();
            self.current_path="".into();
        }
        pub fn now_playing(&self) -> (String, PathBuf) //change to touple (string, duration, pathbuf)
        {
            (self.current.to_string(), self.current_path.clone())
        }
        pub fn elapsed(&self) -> u32
        {
            self.sound.elapsed()
        }
        pub fn get_queue(&self) -> Vec<&str>
        {
            let mut a = Vec::new();
            let mut done = false;
            if self.current==*""
            {
                return a;
            }
            for p in self.queue.split_at(self.qinfo.0).1.iter()
            {
                if a.len()>10
                {
                    done = true;
                    break;
                }
                a.push(p.file_stem().unwrap_or_default().to_str().unwrap_or_default());
            }
            if self.repeat && !done
            {
                let mut pos = 0;
                while a.len()<10
                {
                    a.push(self.queue[pos].file_stem().unwrap_or_default().to_str().unwrap_or_default());
                    pos+=1;
                    if pos >= self.qinfo.1
                    {
                        pos = 0;
                    }
                }
            }
            a
        }
        pub fn shuffle(&mut self)
        {
            use rand::thread_rng;
            use rand::seq::SliceRandom;
            let current = self.current.clone();
            self.queue.shuffle(&mut thread_rng());
            for (i,a) in self.queue.iter().enumerate()
            {
                if *a.file_stem().unwrap().to_str().unwrap().to_string() == current
                {
                    self.qinfo.0 = i;
                }
            }

        }
    }
}
