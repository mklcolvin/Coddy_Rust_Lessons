use crate::playable::Playable;

// TODO: Define a public struct Audio with a public title field (String)

// TODO: Implement the Playable trait for Audio
// The play method should print: Playing audio: {title}

// TODO: Define a public struct Video with a public title field (String)

// TODO: Implement the Playable trait for Video
// The play method should print: Playing video: {title}

pub struct Audio {
    pub title: String,
}

pub struct Video {
    pub title: String,
}

impl Playable for Audio {
    fn play(&self) {
        println!("Playing audio: {}", self.title);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Playing video: {}", self.title);
    }
}
