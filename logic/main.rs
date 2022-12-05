// // //circle of death
// // // use std::io;
// fn main(){
//     // let a=3;
//     // let b=5;
//     // println!("enter the number");
//     // let mut num=string::new();
//     // io::stdin.read_line(&mut num).except("type the number agiain");
//     // let num:f64=num.trim().parse().except("type the number again");
//     let mut sum=0;
//     for i in 1..1000
//     {
//     if i%3==0 || i%5 == 0
//     {
//         sum=sum+i;
        
//     }

//     }  
//     println!("the sum is:{}",sum); 
// }

// sum of even fiboneeci number upto 4 million
//confused aat sum by zero
// fn main(){
//     let mut sum=2;
//     let mut a=1;
//     let mut b=2;
//     let mut c=0;
//     while c<4000000
//     {
//         c=a+b;
        
//         if c%2==0
//         {
//             sum=sum+c;
//         }
//         a=b;
//         b=c;
//     }
//     println!("the sum is:{}",sum);

// }
// to find the prime factor of a number
// use std::io;
// fn main(){
//     let mut num=String::new();
//     println!("enter the number");
//     io::stdin().read_line(&mut num).expect("type the number again");
//     let num:i64=num.trim().parse().expect("type the number again");
//     let mut i=2;
//     while i<num
//     {
//         if num%i==0
//         {
//             println!("the prime factor is:{}",i);
//         }
//         i=i+1;
//     }
// }

//find the  largest prime factor of 600851475143
// fn main(){
//     let mut num:i128=600851475143;
//     let mut i=2;
//     while i<num
//     {
//         if num%i==0
//         {
//             num=num/i;
//         }
//         else
//         {
//             i=i+1;
//         }
//     }
//     println!("the largest prime factor is:{}",num);
// }
//Find the largest palindrome made from the product of two 3-digit numbers.
// fn main(){
//     let mut num=0;
//     let mut a=0;
//     let mut b=0;
//     for i in 100..1000
//     {
//         for j in 100..1000
//         {
//             let mut k=i*j;
//             let mut rev=0;
//             while k>0
//             {
//                 let rem=k%10;
//                 rev=rev*10+rem;
//                 k=k/10;
//             }
//             if rev==i*j
//             {
//                 if rev>num
//                 {
//                     num=rev;
//                     a=i;
//                     b=j;
//                 }
//             }
//         }
//     }
//     println!("the largest palindrome is:{}*{}={}",a,b,num);
// }
// the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
// fn main(){
//     let mut num=1;
//     let mut i=1;
//     while i<20
//     {
//         if num%i==0
//         {
//             i=i+1;
//         }
//         else
//         {
//             num=num+1;
//             i=1;
//         }
//     }
//     println!("the smallest positive number is:{}",num);
// }
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

// fn main()
// {
//     let mut sum=0;
//     let mut sumy=0;
//     for i in 1..101
//     {
//         sum=sum+i;
//         sumy=sumy+i*i;
        
//     }
//     let diff = sum*sum-sumy;
//         println!("{}",diff);
    
// }
// Find the 10001st prime.
// fn main()
// {
//     let mut num=0;
//     let mut i=2;
//     while num <10001
//     {
//         let mut j=2;
//         let mut count=0;
//         while j<i
//         {
//             if i%j==0
//             {
//                 count=count+1;
//             }
//             j=j+1;
//         }
//         if count==0
//         {
//             num=num+1;
//         }
//         num=num+1;
//         i=i+1;
//     }
//     println!("the 10001st prime is:{}",i-1);
// }