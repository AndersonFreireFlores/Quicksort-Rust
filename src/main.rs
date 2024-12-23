fn main() {


}

fn quicksort(x: &mut [i32]) {
    if x.len() < 2{
        return;
    }

    let pivot = partition(x);
    let (left, right) = x.split_at_mut(pivot);
    quicksort(left);
    quicksort(&mut right[1..]);


}

fn partition(arr: &mut [i32]) -> usize{

        let pivot_index = arr.len() / 2;
        arr.swap(pivot_index, arr.len() - 1);

        let mut i = 0;
        for j in 0..arr.len() - 1 {
            if arr[j] <= arr[arr.len() - 1] {
                arr.swap(i, j);
                i += 1;
            }
        }

        arr.swap(i, arr.len() - 1);
        i

}
