
use std::thread;

fn main() {
    let left_half = vec![1,2,3,4];
    let right_half = vec![5,6,7,8];

    let handle1 = thread::spawn(move||{
left_half.into_iter().map(|x|x*x).collect::<Vec<i32>>()
    });


    let handle2 = thread::spawn(move||{
        right_half.into_iter().map(|x| x*x).collect::<Vec<i32>>()
    });

    let res1 = handle1.join().unwrap();
    let res2 = handle2.join().unwrap();

    println!("Результат 1: {:?}", res1); 
println!("Результат 1: {:?}", res2); 

}
