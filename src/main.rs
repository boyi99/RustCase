use core::num;


mod sort_utils {
    
    /*
    @function: bubble_sort()
    @decrption: 冒泡排序  时间复杂度最坏为(O^n^2) 最佳情况是O(n) || 空间复杂度为O（1）
     */
    pub fn bubble_sort(arr: &mut [i32]) {
        let n = arr.len();
        for i in 0..n {
            let mut swapped = false;
            for j in 0..(n - 1 - i) {
                if arr[j] > arr[j+1] {
                    let temp = arr[j];
                    arr[j] = arr[j+1];
                    arr[j+1] = temp;
                    swapped = true;
                }
            }
            if !swapped {
                break;
            }
        }
    }





    /*    
    @function: 
    @decrption: 选择排序 时间复杂度最坏、平均和最佳情况均为O(m^2) 空间复杂度为O(1) 
    @ 
    */

    /*    

    @function: 
    @decrption: 插入排序 时间复杂度最坏、平均和最佳情况均为O(m^2) 空间复杂度为O(1) 
    @ 
    */
    
}



/*
 * @decrption: 冒泡排序
 * @param[in]: 数组
 * @return: 一个顺序排序数组
 */
fn sort_arr(arr:&[i32]) {

}


fn main() {
    let mut numbers = [64, 34, 25, 12, 22, 11, 90];
    print!("原数组: {:?}", numbers);

    sort_utils::bubble_sort(&mut numbers);

    
    print!("原数组: {:?}", numbers);



}
