#[path = "sound.rs"]
mod sound;
#[path = "files.rs"]
mod files;
use files::file;
use sound::sounds::Player;
#[path = "saves.rs"]
mod saves;
use saves::save::Playlist;

pub mod guiplayer
{
    use egui::Id;
    use eframe::egui;
    use crate::gui::Player;
    use crate::gui::Playlist;

    pub struct GUIPlayer
    {
        player: Player,
        show_queue: bool,
        show_playlist: bool,
        playlist: Playlist,
    }

    impl Default for GUIPlayer
    {
        fn default() -> GUIPlayer
        {
            GUIPlayer
            {
                player: Player::new(),
                show_queue: false,
                show_playlist: false,
                playlist: Playlist::new("")
            }
        }
    }

    impl GUIPlayer {
        pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            Self::default()
        }
    }

    impl eframe::App for GUIPlayer {
        fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
            self.player.update();
            let now_playing = self.player.now_playing().0;
            let mut skipnum = 0;
            egui::SidePanel::right(Id::new("right")).show_animated(ctx,self.show_queue, |ui| {
                ui.label("queue:");
                for (i,entry) in self.player.get_queue().iter().enumerate()
                {
                    if ui.small_button(format!("{}: {}",i+1,entry)).clicked()
                    {
                        skipnum = i;
                    }
                }
            });
            for _ in 0..skipnum
            {
                self.player.skip();
            }
            egui::TopBottomPanel::bottom(Id::new("bottom")).show_animated(ctx,self.show_playlist, |ui| {
                ui.label("playlist");
                if ui.button("load playlist").clicked()
                {
                    self.playlist = Playlist::load();
                }
                if ui.button("save playlist").clicked()
                {
                    self.playlist.save();
                }
                if ui.button("add to playlist").clicked()
                {
                    self.playlist.add(self.player.now_playing().1);
                }
                if ui.button("remove from playlist").clicked()
                {
                    self.playlist.remove(self.player.now_playing().1);
                }
                if ui.button("play playlist").clicked()
                {
                    self.player = Player::new();
                    for path in self.playlist.get_music_files()
                    {
                        self.player.add(path);
                    }
                }
                let mut s = self.playlist.get_name();
                ui.text_edit_singleline(&mut s);
                self.playlist.set_name(&s);
                let list = self.playlist.get_music_files().iter().map(|p| p.file_stem().unwrap_or_default().to_str().unwrap_or_default()).collect::<Vec<&str>>().join(" ");
                ui.label(list);
            });
            egui::CentralPanel::default().show(ctx, |ui| {
                if ui.button("Folder").clicked()
                {
                    use crate::gui::file::get_music_files;
                    use crate::gui::file::get_folder;
                    if let Some(a) = get_folder() {
                        let folder = a;
                        self.player.reset();
                        for name in get_music_files(folder)
                        {
                            self.player.add(name);
                        }
                    }
                }
                if ui.button(format!("show queue: {}",self.show_queue)).clicked()
                {
                    self.show_queue = !self.show_queue;
                }
                if ui.button(format!("show playlist: {}",self.show_playlist)).clicked()
                {
                    self.show_playlist = !self.show_playlist;
                }
                if ui.button("previous").clicked()
                {
                    self.player.prev();
                }
                if ui.button("skip").clicked()
                {
                    self.player.skip();
                }
                if ui.button("Pause/play").clicked()
                {
                    self.player.play_pause();
                }
                let t = self.player.elapsed();
                ui.label(format!("Now playing: {now_playing}"));
                if now_playing != *""
                {
                    let mut seek = t;
                    ui.add(egui::Slider::new(&mut seek, 0..=self.player.get_duration())
                    .custom_formatter(|n, _| {let n = (n/1000.0) as i32;let hours = n / (60 * 60);let mins = (n / 60) % 60;let secs = n % 60;format!("{hours:02}:{mins:02}:{secs:02}")})).clicked();
                    if seek != t
                    {
                        self.player.seek(seek);
                    }
                }
                if ui.button("shuffle").clicked()
                {
                    self.player.shuffle()
                }
                if ui.button(format!("repeat: {}",self.player.get_repeat())).clicked()
                {
                    self.player.toggle_repeat();
                }
            });
            if self.player.now_playing().0 != *"" || self.player.elapsed() != self.player.get_duration()
            {
                ctx.request_repaint();
            }
        }
    }
}

