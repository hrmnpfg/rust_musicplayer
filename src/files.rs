pub mod file
{
    use std::fs::{self, ReadDir};
    use std::path::*;
    pub struct FileIterator
    {
        dirs: Vec<PathBuf>,
        files: Option<ReadDir>
    }

    impl From<&str> for FileIterator
    {
        fn from(path: &str) -> Self
        {
            FileIterator
            {
                dirs: vec![PathBuf::from(path)],
                files: None,
            }
        }
    }

    impl Iterator for FileIterator
    {
        type Item = PathBuf;
        fn next(&mut self) -> Option<PathBuf>
        {
            while let Some(read_dir) = &mut self.files
            {
                match read_dir.next()
                {
                    Some(Ok(entry)) => {
                        let path = entry.path();
                        if let Ok(md) = entry.metadata()
                        {
                            if md.is_dir()
                            {
                                self.dirs.push(path.clone());
                                continue;
                            }
                        }
                        return Some(path);
                    }
                    None => {
                        self.files = None;
                        break;
                    }
                    _ => { }
                }
            }
            while let Some(dir) = self.dirs.pop()
            {
                let read_dir = fs::read_dir(&dir);
                if let Ok(files) = read_dir
                {
                    self.files = Some(files);
                    return Some(dir);
                }
            }
            None
        }
    }

    use rfd::FileDialog;
    pub fn get_folder() -> Option<PathBuf>
    {
        FileDialog::new().set_directory("/").pick_folder()
    }
    pub fn get_file() -> Option<PathBuf>
    {
        FileDialog::new().set_directory("/").pick_file()
    }

    pub fn get_music_files(p: PathBuf) -> Vec<PathBuf>
    {
        let mut music = Vec::new();
        for f in FileIterator::from(p.to_str().unwrap())
        {
            let binding = f.file_name().unwrap().to_str().unwrap().to_string();
            let ext = binding.split('.').last().unwrap();
            match ext
            {
                "mp3" => {music.push(f)},
                "flac" => {music.push(f)},
                "wav" => {music.push(f)},
                _ => ()
            }
        }
        music
    }
}
