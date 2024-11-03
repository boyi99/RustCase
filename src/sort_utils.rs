
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
    @function: selection_sort()
    @decrption: 选择排序 时间复杂度最坏、平均和最佳情况均为O(m^2) 空间复杂度为O(1) 
    @ 
    */
    pub fn selection_sort(arr: &mut [i32]) {
        let n:usize = arr.len();
        for i in 0..n {
            // 记录最小元素索引，假设i为最小元素对应的索引
            let mut min_index = i;
            for j in (i+1)..n {
                if arr[j] < arr[min_index] {
                    min_index = j;
                }
            }
            if min_index!=i {
                let temp = arr[i];
                arr[i] = arr[min_index];
                arr[min_index] = temp;
            }
        }
    }


    /*    
    @function: insert_sort()
    @decrption: 插入排序
    @param[in] : &mut [i32]
    */
    pub fn insert_sort(arr: &mut [i32]) {

    }



    /*    
    @function: merge_sort()
    @decrption: 归并排序
    @param[in] : &mut [i32]
    */
    pub fn merge_sort(arr: &mut [i32]) {

    }


    /*    
    @function: quick_sort()
    @decrption: 快速排序
    @param[in] : &mut [i32]
    */
    pub fn quick_sort(arr: &mut [i32]) {

    }


    /*    
    @function: heap_sort()
    @decrption: 堆排序
    @param[in] : &mut [i32]
    */
    pub fn heap_sort(arr: &mut [i32]) {

    }


    /*    
    @function: counting_sort()
    @decrption: 计数排序
    @param[in] : &mut [i32]
    */
    pub fn counting_sort(arr: &mut [i32]) {

    }



    /*    
    @function: radix_sort()
    @decrption: 基数排序
    @param[in] : &mut [i32]
    */
    pub fn radix_sort(arr: &mut [i32]) {

    }



    /*    
    @function: bucket_sort()
    @decrption: 桶排序
    @param[in] : &mut [i32]
    */
    pub fn bucket_sort(arr: &mut [i32]) {

    }