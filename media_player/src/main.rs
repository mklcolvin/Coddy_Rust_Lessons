mod playable;
mod media;

use media::{Audio, Video};
use playable::Playable;

fn main() {
    // Read input
    let mut audio_title = String::new();
    std::io::stdin().read_line(&mut audio_title).expect("Failed to read line");
    let audio_title = audio_title.trim().to_string();
    
    let mut video_title = String::new();
    std::io::stdin().read_line(&mut video_title).expect("Failed to read line");
    let video_title = video_title.trim().to_string();
    
    // TODO: Create an Audio instance with audio_title
    let audio = Audio { title: audio_title };
    
    // TODO: Create a Video instance with video_title
    let video = Video { title: video_title };
    
    // TODO: Call play() on both instances
    audio.play();
    video.play();
}
