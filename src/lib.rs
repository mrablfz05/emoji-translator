use wasm_bindgen::prelude::*;
use std::collections::HashMap;
// use web_sys::console;

#[wasm_bindgen]
// Translate words into emoji
pub fn emoji_translator(text: &str) -> String {
    let mut emoji_dict: HashMap<&str, &str> = HashMap::new();

    emoji_dict.insert("happy", "😀");
    emoji_dict.insert("sad", "😢");
    emoji_dict.insert("love", "❤️");
    emoji_dict.insert("fire", "🔥");
    emoji_dict.insert("cool", "😎");
    emoji_dict.insert("angry", "😠");
    emoji_dict.insert("cry", "😭");
    emoji_dict.insert("laugh", "😂");
    emoji_dict.insert("coffee", "☕");
    emoji_dict.insert("heart", "💖");
    emoji_dict.insert("sun", "☀️");
    emoji_dict.insert("moon", "🌙");
    emoji_dict.insert("money", "💰");

    // println!("Test Emoji Value: {:?}", emoji_dict);
    // console::log_1(&format!("My emoji console.log: {:?}", emoji_dict).into());
    
    // Convert text into words and replace them with emojis
    text.split_whitespace()
    .map(|word| emoji_dict.get(word).unwrap_or(&word).to_string())
    .collect::<Vec<String>>()
    .join(" ")
}