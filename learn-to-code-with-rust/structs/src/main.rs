struct TaylorSong {
    title: String,
    released_year: u32,
    duration_sec: u32,
}

impl TaylorSong {
    fn display_song_info(&self) {
        // immutable option passing the values but they are immutable and receive Ownership
        // mutable passing the values but they are mutable and receive Ownership
        // Immutable reference to the struct instance (not taking ownership)
        // Mutable reference

        println!("Title: {}", self.title);
        println!("Released: {}", self.released_year);
        println!("duration: {}", self.duration_sec);
    }

    fn double_duration(mut self) {
        self.duration_sec *= 2;
    }
}

fn main() {
    let song = TaylorSong {
        title: String::from("Blank space"),
        released_year: 2020,
        duration_sec: 200,
    };

    song.display_song_info();
}
