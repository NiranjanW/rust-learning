fn main() {
    let   mut arr = [6,3,1,7,2];
    insert_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

fn insert_sort( arr : &mut [i32] ) {
    let mut i = 1;
    while i < arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }

    fn insertion_sort1(arr: &mut [i32]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j - 1] > arr[j] {
                arr.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}
