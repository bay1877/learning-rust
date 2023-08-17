use tts_rust::{ tts::GTTSClient, languages::Languages };

fn main() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    narrator.speak("Hello, World!");
}