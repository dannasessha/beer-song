pub fn verse(n: u32) -> String {
    if n == 0 {
        "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string()
    } else if n == 1 {
        "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string()
    } else if n == 2 {
        "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string()
    } else {
	format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n - 1)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut compound_song: String = "".to_string();
    let mut i: u32 = start;
    while i >= end {
        let mut compound_verse: String = "".to_string();
        if i < start {
            compound_verse.push_str(&"\n");
        } 
        compound_verse.push_str(&verse(i));
        compound_song.push_str(&compound_verse);
        if i > 0 {
            i -= 1;
        } else {
            break;
        }
    }
    compound_song
}
