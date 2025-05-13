
fn buble_sort(arr :&mut Vec<i32>){
    let l:usize=arr.len();

    for i in 0..l-1{
        for j in i+1..l{
            if arr[i]>arr[j]{
                arr.swap(i,j);
            }
        }
    }
}

fn insertion_sort(arr :&mut Vec<i32>){
    let l:usize=arr.len();

    for i in 1..l{
        let mut j=i;
        while j>0 && arr[j]<arr[j-1]{
            arr.swap(j,j-1);
            j-=1;
        }
    }
}

fn selection_sort(arr :&mut Vec<i32>){
    let l:usize=arr.len();
    let mut smallest_index=0;

    for i in 0..l-1{
        smallest_index=i;
        for j in i+1..l{
            if arr[smallest_index]>arr[j]{
                smallest_index=j;
            }
        }
        if smallest_index!=i{
            arr.swap(i,smallest_index);
        }
    }
}

fn merge_sort(arr :&mut Vec<i32>){

}

fn quick_sort(arr :&mut Vec<i32>){
    
}

fn main() {
    let mut arr:Vec<i32>=[8,1,5,7,4,3,9,2,0,6].to_vec();
    
    println!("{:?}",arr);

    selection_sort(&mut arr,l);

    println!("{:?}",arr);
}
