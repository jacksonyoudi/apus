struct Audio(String);

struct Video(String);


trait Playable {
    fn play(&self);
    fn pause() {
        println!("Paused");
    }
}

impl Playable for Audio {
    fn play(&self) {
        println!("Now playing:{}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}


trait Vehichle {
    fn get_price(&self) -> u64;
}

trait Car: Vehichle {
    fn model(&self) -> String;
}

struct TeslaRoadster {
    model: String,
    release_date: u16,
}

impl TeslaRoadster {
    fn new(model: &str, release_date: u16) -> Self {
        Self {
            model: model.to_string(),
            release_date: release_date,
        }
    }
}


impl Vehichle for TeslaRoadster {
    fn get_price(&self) -> u64 {
        200_000
    }
}

impl Car for TeslaRoadster {
    fn model(&self) -> String {
        "Tesla Roadster I".to_string()
    }
}
