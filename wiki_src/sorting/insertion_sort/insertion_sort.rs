fn insertion_sort<T>(a: &mut [T]) 
    where T: Ord
{
    for i in 1..a.len() {
        let mut j = i;
        while j > 0 && a[j-1] > a[j] {
            a.swap(j, j-1);
            j -= 1;
        }
    }
}
