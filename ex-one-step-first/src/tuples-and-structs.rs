fn main(){

    // tuple istalgan tipdagi 
    // qiymatlarni saqlaydi, har hil turdagi qiymatlarni funksiyadan yuborish uchun
    // tuple dan foydalanish yaxshi ekan lekin tuple o'zgarmaydi,
    let tuple_e = ('E', 8i32, true);

    println!("Elements, 1 = {}, 2 = {}, 3 = {}", tuple_e.0, tuple_e.1, tuple_e.2);
}