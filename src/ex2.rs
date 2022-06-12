/*
Bài tập 2:
    Cho 1 chuỗi str Slice như dưới đây.
    Nhập 1 từ bất kỳ từ bàn phím, in ra số lượng từ này xuất hiện trong chuỗi đã cho.
    Nâng cao hơn : Tìm kiếm không phân biêt chữ hoa thường, theo dạng regex.
    https://ars.els-cdn.com/content/image/1-s2.0-S0960982203005347-mmc6.txt

    Ví dụ: cho chuỗi str = “This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.”
    Nhập từ bàn phím : “is is” => In ra kết quả số lượng “is is” là 5
 */


pub(crate) fn run() {
    println!();
    println!("[Exercise 2] Starting...");

    let text = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";
    let find_text = "is is";

    let mut count = 0;

    let text_chars: Vec<char> = text.chars().collect();
    let find_chars: Vec<char> = find_text.chars().collect();

    let mut i = 0;
    while i < text_chars.len() {
        if text_chars[i].to_lowercase().to_string() == find_chars[0].to_lowercase().to_string() {
            if find_chars.len() == 1 {
                count += 1;
                i += 1;
                continue;
            }
            for j in 1..find_chars.len() {
                if i + j >= text_chars.len() {
                    break;
                }
                if text_chars[i + j].to_lowercase().to_string() != find_chars[j].to_lowercase().to_string() {
                    i += 1;
                    break;
                }
                if j == find_chars.len() - 1 {
                    // Found
                    count += 1;
                    i += j; // Start from the end of current found char.
                }
            }
        }
        i += 1;
    }

    println!(">> Found {} time(s): '{}'", count, find_text)
}