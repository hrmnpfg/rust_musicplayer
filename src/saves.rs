pub mod save
{
    use std::path::PathBuf;
    use crate::gui::file::get_file;
    use crate::gui::file::get_folder;
    use std::fs;
    use std::io::Write;

    pub struct Playlist
    {
        name: String,
        contents: Vec<PathBuf>,
        location: Option<PathBuf>,
        filename: String
    }

    impl Playlist
    {
        pub fn new(name: &str) -> Playlist
        {
            Playlist
            {
                name: name.to_string(),
                contents: Vec::new(),
                location: None,
                filename: name.to_string()
            }
        }
        pub fn load() -> Playlist
        {
            let p = get_file();
            let mut pl = Playlist::new("");
            let p2 = p.clone();
            match p
            {
                Some(a) => {
                    pl.location = Some(a.parent().unwrap().to_path_buf());
                    pl.filename = a.file_name().unwrap().to_str().unwrap_or_default().to_string();
                },
                None => return pl
            }
            let f = fs::read_to_string(p2.unwrap()).unwrap();
            for (i,l) in f.split('\n').enumerate() {
                if i==0
                {
                    pl.name = l.to_string();
                    continue;
                }
                let o = PathBuf::from(l);
                if o.is_file()
                {
                    pl.contents.push(o);
                }
            }
            pl
        }
        pub fn save(&mut self)
        {
            if self.location.is_none()
            {
                let p = get_folder();
                match p
                {
                    Some(a) => self.location = Some(a),
                    None => return
                }
            }
            let f = fs::File::options().create(true).truncate(true).write(true).open(self.location.clone().unwrap().to_str().unwrap().to_owned() + "/"+&self.filename);
            let mut file;
            match f
            {
                Ok(a) => {
                    file = a;
                },
                Err(_) => return
            }
            file.write((self.name.clone()+"\n").as_bytes()).ok();
            for s in self.contents.iter()
            {
                file.write((s.to_str().unwrap_or_default().to_owned()+"\n").as_bytes()).ok();
            }
        }
        pub fn get_name(&self) -> String
        {
            self.name.clone()
        }
        pub fn set_name(&mut self, s: &str)
        {
            self.name = s.to_string();
            if self.filename == *""
            {
                self.filename = s.to_string()
            }
        }
        pub fn add(&mut self, path: PathBuf)
        {
            if self.contents.contains(&path)
            {
                return;
            }
            self.contents.push(path);
        }
        pub fn remove(&mut self, track: PathBuf)
        {
            let track_str = track.to_str().unwrap();
            let mut to_remove = (0, false);
            for (i,t) in self.contents.iter().enumerate()
            {
                if t.to_str().unwrap() == track_str
                {
                    to_remove = (i,true);
                    break;
                }
            }
            if to_remove.1
            {
                self.contents.remove(to_remove.0);
            }
        }
        pub fn get_music_files(&self) -> Vec<PathBuf>
        {
            self.contents.clone()
        }
    }
}
