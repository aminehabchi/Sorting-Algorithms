
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



fn merge_sort(arr :&mut [i32])->Vec<i32>{
    let l = arr.len();
    if l <= 1 {
        return arr.to_vec();
    }

    let left = merge_sort(&mut arr[0..l/2]);
    let right = merge_sort(&mut arr[l/2..]);

    let mut ar:Vec<i32>=Vec::new();

    let mut i:usize=0;
    let mut j:usize=0;

    loop {
        if i<left.len() && j<right.len(){
            if left[i]<right[j]{
                ar.push(left[i]);
                i+=1;
            }else{
                ar.push(right[j]);
                j+=1;
            }
        }else if i<left.len(){
            ar.push(left[i]);
            i+=1;
        }else if j<right.len(){
            ar.push(right[j]);
            j+=1;
        }else{
            break
        }
    }

    ar
}

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
    let mut arr:Vec<i32>=[5,1,2,0,8,8,6,9,4].to_vec();
    
    println!("{:?}",arr);

    arr=merge_sort(&mut arr);

    println!("{:?}",arr);
}
