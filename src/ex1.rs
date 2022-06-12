/*
Bài tập 1:  Cho 2 mảng có các phần tử là số nguyên dương.
            Kiểm tra mảng này có phải là mảng con của mảng kia không?
            (yêu cầu đúng thứ tự của các phần tử)

Ví dụ : let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
        let sub_arr = [6, 8, 10];
 */


pub(crate) fn run() {
    println!("[Exercise 1] Starting...");

    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];

    println!("Org arr: {:?}", org_arr);
    println!("Sub arr: {:?}", sub_arr);

    let mut is_contains = false;
    let first_sub_numb = sub_arr[0];

    for org_index in 0..org_arr.len() {
        // Tìm kiếm phần tử đầu tiên trong org_arr bằng với 0 ở sub.arr[0]
        if org_arr[org_index] == first_sub_numb {
            // Check từng phần tử tiếp theo trong sub_arr với org_arr theo đúng thứ tự
            for sub_index in 1..sub_arr.len() {
                // break khỏi for loop khi giá trị ko bằng nhau
                if org_arr[org_index + sub_index] != sub_arr[sub_index] {
                    break;
                }

                // Khi toàn bộ phần tử ở sub_arr đều bằng ở org_arr, theo thứ tự
                if sub_index == sub_arr.len() - 1 {
                    is_contains = true;
                }
            }
        }
        if is_contains {
            break;
        }
    }

    println!("Result:  {:?} {} {:?}", org_arr, (if is_contains {"CONTAINS"} else {"DOESN'T CONTAINS"}), sub_arr)
}