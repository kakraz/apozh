mod audio;

slint::include_modules!();

fn main() {
    let mut audio_helper = audio::get_audio_helper();
    audio_helper.play_file("audio/lyutyy-negr-oret.mp3");
    MainWindow::new().unwrap().run().unwrap();
}