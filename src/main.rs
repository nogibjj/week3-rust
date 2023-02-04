fn main(){
    const N: i32=100;
    let mut count=0;
    for i in 1..N{
        let mut temp=true;
        for j in 2..i/2+1{
            if i%j==0 {
                temp=false;
                break;
            }    
        }
        if temp{
            count+=1;
            print!("{} ",i);
            if count%5==0 {
                println!();
            }
        }
    }
}