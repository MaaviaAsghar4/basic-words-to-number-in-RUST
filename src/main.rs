fn main() {
    let basic_num = [
        "zero","one","two","three","four","five","six","seven","eight","nine","ten","eleven","twelve","thirteen","fourteen","fifteen","sixteen","seventeen","eighteen","ninteen"
        ];
    let basic_tens = [
        "twenty","thirty","forty","fifty","sixty","seventy","eighty","ninty"
    ];
    let basic_hundred = [
        "one hundred","two hundred","three hundred","four hundred","five hundred","six hundred","seven hundred","eight hundred","nine hundred"
    ];
    let my_num = 121.0;
    if my_num < 20.0 {
        println!("Your number is {}",basic_num[my_num as usize])
    } else if my_num > 19.0 && my_num <= 99.0 {
        let tens = (my_num as f64/10.0).floor();
        let units = my_num as f64 % 10.0;
        if units == 0.0 {
            println!("Your number is {}",basic_tens[tens as usize - 2])
        } else {
            println!("Your number is {} {}",basic_tens[tens as usize - 2],basic_num[units as usize])
        }
    } else if my_num > 99.0 && my_num <= 999.0 {
        let hundred = (my_num as f64 / 100.0).floor();
        let get_tens = my_num - (&hundred) * 100.0; 
        let tens = (get_tens as f64/10.0).floor();
        let units = get_tens as f64 % 10.0;
        if tens == 0.0 && units == 0.0 {
            println!("Your number is {}",basic_hundred[hundred as usize - 1]);
        } else if get_tens > 0.0 && get_tens < 20.0 {
            println!("Your number is {} {}",basic_hundred[hundred as usize - 1],basic_num[get_tens as usize]);
        } else if units == 0.0 {
            println!("Your number is {} {}",basic_hundred[hundred as usize - 1],basic_tens[tens as usize - 2]);
        } else {
            println!("Your number is {} {} {}",basic_hundred[hundred as usize - 1],basic_tens[tens as usize - 2],basic_num[units as usize]);
        }
    }
}
