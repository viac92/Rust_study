use trait_lib::{ Summary, Tweet};

struct Song {
    lyric: String,
    author: String,
}

// we can implement our Summary trait for the local struct 
impl Summary for Song {
    fn summarize(&self) -> String {
        format!("{} {}", self.author, self.lyric)
    }
}


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    let song = Song {
        lyric: String::from("Karma police..."),
        author: String::from("Radiohead"),
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new song: {}", song.summarize());
}
