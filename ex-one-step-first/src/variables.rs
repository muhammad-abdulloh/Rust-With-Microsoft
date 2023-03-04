fn main(){
    /* manashu holatda error beradurg'o'y

    // o'zgaruvchilar default holatda constanta o'zgartirib bo'lmas 
    // bo'lara ekan
    // variable inmutable

    let number = 23;
    number = 45;

    println!("Toza error, {}", number);

    */

/*
// o'zgaruvchini mutable qiganimizda bitta underline qo'yishimiz kerak ekan
// bo'masam warning berib yotibdi ishkalzo'vit
    let mut _number = 20;
    _number = 34;

    println!("Bu ishledi endi o'zgaruvchimizni mutable qilib qo'ydik, {}", _number);

*/

// bitta nomdagi o'zgaruvchini qayta qayta elon qilish mumkin ekan :)
// welocme to Rust :)
    let number = 5;
    let number = number + 5;
    let number = number*2;

    println!("Ajoyib: {}",number);
}