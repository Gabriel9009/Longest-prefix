fn main() {
    let mut count: usize = 0;
    let mut count1: usize = 0;
    let  vec = vec!["flower", "flow", "flight"];
    for _i in 0..vec.len(){
      loop{
         if vec[count].chars().nth(count1) == vec[count + 1].chars().nth(count1){
           count += 1;
           if count == 2{
            println!("{:?}", vec[count].chars().nth(count1));
            count = 0;
            count1 += 1;
          }
        }else{ 
          break;
        }
      }
    }
  }