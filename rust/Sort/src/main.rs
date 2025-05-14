
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
    
    for i in 0..l-1{
        let mut smallest_index=0;
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

// fn merge_sort(arr1 :&mut Vec<i32>,arr1 :&mut Vec<i32>){
    
// }

fn quick_sort(arr :&mut [i32]){
    if arr.len()<=1{
        return;
    }

    let l:usize=arr.len();

    let privot :i32=arr[0];
    
    let mut j=1;

    for i in 1..l{
        if arr[i]<privot{
            arr.swap(i,j);
            j+=1;
        }
    }
    arr.swap(j-1, 0); 

    quick_sort(&mut arr[0..j-1]);
    quick_sort(&mut arr[j..l]);
}



fn main() {
    let mut arr:Vec<i32>=[5,1,2,0,6,9,4].to_vec();
    
    println!("{:?}",arr);

    quick_sort(&mut arr);

    println!("{:?}",arr);
}
