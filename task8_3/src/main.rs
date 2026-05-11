fn main() {
    
let mut nums = vec![1,2,3];

for i in & mut nums{
    *i =*i*2;
}

println!("Vector: {:?}", nums);

}
