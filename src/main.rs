


mod sort_utils;

fn main() {
    let mut numbers = [64, 34, 25, 12, 22, 11, 90];
    print!("原数组: {:?}", numbers);

    //sort_utils::bubble_sort(&mut numbers);
    sort_utils::selection_sort(&mut numbers);

    
    print!("原数组: {:?}", numbers);



}
