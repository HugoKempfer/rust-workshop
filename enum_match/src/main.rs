struct User {
    name: String,
    mail: String,
    age: u32,
    picture: Image,
    bio: Option<String>,
}

impl User {
    fn new(name: String, mail: String, age: u32, picture: Image, bio: Option<String>) -> User {
        User {
            name,
            mail,
            age,
            picture,
            bio,
        }
    }
}

struct Image {
    name: String,
    url: Vec<u8>,
}

impl Image {
    fn new(name: String, url: Vec<u8>) -> Image {
        Image { name, url }
    }
}

enum Message {
    MessageContent(String),
    Image(Image),
    SharedContact(User),
}

fn register(_contact: User) {}

fn fetch_message() -> Option<Message> {
    Some(Message::MessageContent(
        "Le message est ici une string mais aurait pu être une image ou un contact".to_string(),
    ))
}
fn save_image_url(_url: Vec<u8>) {}

fn main() {
    if let Some(message) = fetch_message() {
        match message {
            Message::MessageContent(message_string) => println!("{}", message_string),
            Message::Image(image) => save_image_url(image.url),
            Message::SharedContact(contact) => register(contact),
        }
    }
}
