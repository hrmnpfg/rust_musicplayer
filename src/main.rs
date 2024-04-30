mod gui;
use crate::gui::guiplayer::GUIPlayer;

fn main()
{
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("Music player", native_options, Box::new(|cc| Box::new(GUIPlayer::new(cc))));
}
