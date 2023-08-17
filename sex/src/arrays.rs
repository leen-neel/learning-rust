fn main() {
    
    let arr1:[i32;5] = [1, 2, 3, 4, 5];
    println!("{}", arr1[0]);
    
    let slice_arr:&[i32] = &arr1[0..2];
    println!("{:?}", slice_arr)

}
