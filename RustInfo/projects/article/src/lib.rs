#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        {
            use crate::online::{Tweet, NewsArticle, Summary, ZonePost};
            let tweet = Tweet {
                username: "John James".to_string(),
                content: String::from("It's just a short tweet with 280 words limit!"),
                reply: false,
                retweet: false,
            };
            let news = NewsArticle {
                headline: String::from("The world is changing so fast"),
                location: String::from("Los Angle"),
                author: "Kobe Bryant".to_string(),
                content: String::from("Of course, you probably already know, the world is changing so fast that it's hard to catch..."),
            };

            let zone = ZonePost {
                content: String::from("Just a test"),
                at: "Leborn James".to_string(),
            };
            assert_eq!(news.summary(), String::from("The world is changing so fast, by Kobe Bryant Los Angle"));
            assert_eq!(tweet.summary(), String::from("John James: It's just a short tweet with 280 words limit!"));
            assert_eq!(zone.summary(), String::from("(read more from @Leborn James...)"));
        }
    }
}

// Article online
pub mod online {
    pub trait Summary {
        // Default for trait method
        fn summary(&self) -> String {
            format!("(read more from {}...)", self.summary_author())
        }

        fn summary_author(&self) -> String;
    }
    // News article type
    #[derive(Debug)]
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    #[derive(Debug)]
    pub struct ZonePost {
        pub content: String,
        pub at: String,
    }

    // Tweet limits words of 0-280
    #[derive(Debug)]
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for NewsArticle {
        fn summary(&self) -> String {
            format!("{}, by {} {}", self.headline, self.author, self.location)
        }

        fn summary_author(&self) -> String {
            format!("@{}", self.author)
        }
    }

    impl Summary for Tweet {
        fn summary(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }

        fn summary_author(&self) -> String {
            format!("@{}", self.username)
        }
    }

    impl Summary for ZonePost {
        fn summary_author(&self) -> String {
            format!("@{}", self.at)
        }
    }

}
