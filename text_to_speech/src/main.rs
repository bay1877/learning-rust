use tts_rust::{ tts::GTTSClient, languages::Languages };

fn main() {
    let mut narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::English, // use the Languages enum
        tld: "com",
    };
    narrator.speak("Hello, World!");

    let mut shopping_list = Vec::new();
    shopping_list.push("Apples");
    shopping_list.push("Bananas");
    shopping_list.push("Chocolate");

    for item in shopping_list.iter(){
        narrator.speak(item);
    }

}