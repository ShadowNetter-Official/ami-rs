fn main() {
    let messages = [
        "depressed",
        "happy", 
        "sad", 
        "angry", 
        "excited", 
        "anxious", 
        "calm", 
        "confused", 
        "surprised", 
        "frustrated", 
        "hopeful", 
        "lonely", 
        "proud", 
        "jealous", 
        "grateful", 
        "embarrassed", 
        "relieved", 
        "guilty", 
        "nervous", 
        "overwhelmed"
    ];

    let message = messages[fastrand::usize(..messages.len())];
    println!("{}", message);
}
