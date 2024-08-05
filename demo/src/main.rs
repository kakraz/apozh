mod audio;

slint::include_modules!();

fn main() {
    let mut audio_helper = audio::get_audio_helper();
    let main_window = MainWindow::new().unwrap();
    main_window.on_play_audio_file(move || {
        audio_helper.play_file("audio/lyutyy-negr-oret.mp3");
    });
    main_window.run().unwrap();
}