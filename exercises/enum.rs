// Make me compile.

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}


fn inspect(event: WebEvent) {
    match event {
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        // implement all other variants of the enum with appropriate output
    }
}


fn main() {
    inspect(WebEvent::PageLoad);
    inspect(WebEvent::PageUnload);
    inspect(WebEvent::KeyPress('a'));
    inspect(WebEvent::Paste("ABC".to_owned()));
    inspect(WebEvent::Click {x: 1, y: 2});
}
