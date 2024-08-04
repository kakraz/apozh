mod audio;

fn main() {
    let mut audio_helper = audio::get_audio_helper();
    audio_helper.play_file("audio/lyutyy-negr-oret.mp3");
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    component MemoryTile inherits Rectangle {
        width: 600px;
        height: 600px;
        background: #3960D5;

        Image {
            source: @image-url("pictures/licensed-image.jpeg");
            width: parent.width;
            height: parent.height;
        }
    }


    export component MainWindow inherits Window {
        MemoryTile {}
    }
}
