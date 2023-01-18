// use ferris_says::say; // from the previous step
// use std::io::{stdout, BufWriter};

//use core::num::dec2flt::number;

use std::iter::Sum;

fn main() {
    println!("Hello, world!");

    //match

    let learn_lang = "Rust";

    match learn_lang {
        "js" => println!("We are learning Rust"),
        "html" => println!("We are learning html"),
        _ => println!("We are learning {}", learn_lang),
    }

    let sides: u8 = 7;
    let figure: String;

    figure = match sides {
        1 => "Line".to_string(),
        2 => "Pryamougolnik".to_string(),
        3 => "Triangle".to_string(),
        4 => "Quadroangle".to_string(),
        5 => "Fiveangle".to_string(),
        _ => format!("{}-ugolnik", sides),
    };

    print!("Yor figure {}", figure);

    //match close

    let mut x = 5;

    println!("The value of x is: {}", x);

    x = 6;

    println!("The value of x is: {}", x);

    println!("Hello, world! My first program!");

    // let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();

    println!("Hello world v.2!");

    println!("Сумма 3 + 5 = {}", 8);

    println!("Сумма 3 + 5 = {}", 3 + 5);

    println!("Не приложишь {}, не появится {}.", "труд", "пруд");

    println!(
        "Не стыдно не {0} стыдно не {1}. Давайте {1}. ",
        "знать", "учиться"
    );

    println!(
        "Студент {name} сдал экзамен на {grade}.",
        grade = 5,
        name = "Вася"
    );

    let n = 3 + 5;
    println!("Сумма 3 + 5 = {n}");

    let st: String;
    st = format!("Три и пять равно 8");
    println!("{}", st);

    println!("{:?}", vec!["a", "b", "c"]);

    println!("{:#?}", vec![Some("Hello"), None, Some("World")]);

    let v = 80;
    let t = 2;
    println!(
        "Автомобиль ехал 2 часа со скоростью 80 км/ч и проехал {} km",
        v * t
    );

    let d1 = "один дракон";
    let d2 = "два двухголовых дракона";
    let d3 = "три трехголовых дракона";

    println!("{}", d1);
    println!("{0} {0}", d2);
    println!("{0} {0} {0}", d3);

    let number = 239;
    let company = "'Coca-cola'";
    let worker = "'Itech'";

    println!(
        "Договор № {number}
    {company} именуемое в дальнейшем «Заказчик», 
    и {worker}, именуемое в дальнейшем «Подрядчик», совместно именуемые «Стороны», 
    заключили настоящий договор.
    Текст договора
    Подписи:
    Заказчик: {company}    Подрядчик: {worker}"
    );

    let mut a = 4;
    let mut b = 3;
    a = a + b;
    a = a * b;
    a = a - b;
    b = b - a;
    println!("a:{}", a);
    println!("b:{}", b);

    let mut a = true;
    let mut b = true;
    a = a > b && b < a;
    b = !b;
    println!("a: {}", a);
    println!("b: {}", b);

    /* Дано трехзначное число. Напишите программу, которая вычислит
    и выведет в консоль все цифры этого числа.*/

    let n = 132;
    println!("{}{}{}", n / 100, n % 100 / 10, n % 10);

    let n = 293;
    println!(
        "Sotni = {}\nDec = {}\nEdinic = {}",
        n / 100,
        n % 100 / 10,
        n % 10
    );

    let width = 10;
    let hight = 5;
    let dlina = 15;

    println!(
        "S potolka = {}, S sten = {}",
        width * dlina,
        (width * hight + dlina * hight) * 2
    );

    let d = 28.0;
    let s = 7.0;
    let cm = d * 2.54;
    let km: f64 = s * 1000.0 * 100.0;
    println!("Kolichestvo oborotov = {:.1}", km / (cm * 3.14));

    let mut st1: f64;
    st1 = 23.0_f64;
    let st2 = 46.0_f64;
    println!("Diagonal = {:.1}", (st1.powf(2.0) + st2.powf(2.0)).sqrt());

    st1 = 33.0;
    println!("{}", st1);

    let m = 7;
    let n = 2;
    if m % n == 0 {
        println!("rez {}", m / n);
    } else {
        println!("{m} ne delitsa na {n}");
    }

    let c37 = 126;
    if c37 < 0 {
        println!("otric");
    } else if (c37 % 3 == 0) || (c37 % 7 == 0) {
        println!("delitsya");
    } else {
        println!("ne delitsya");
    }

    let count_v = 5;
    let count_mod: &str = match count_v {
        5 => "Pytiugolnik",
        6 => "Shestiugolnik",
        7 => "Semiugolnik",
        8 => "Vismitiugolnik",
        _ => "Mnogougolnik",
    };
    println!("Vasha figura: {}", count_mod);

    let mut summ = 0;
    for i in 17..=24 {
        summ += i;
    }
    println!("{} ", summ);

    let mut n: i8 = 14;
    let mut temp_info = 0;
    while n >= 0 {
        temp_info = n;
        n -= 5;
    }
    println!("{temp_info}");

    /*Доработайте эту программу, чтобы она распечатала остаток
    от деления n на 5. Не используйте оператор %. */

    // const START: u64 = 37;
    // const PRODUCTIVITY: u64 = 111;
    // const SURVIVAL: u64 = 60; // процентов
    // let period: i8 = 4;
    // let mut result: u64 = START;

    // for i in 1..=period {
    //     result = result + result * PRODUCTIVITY * SURVIVAL / 100;
    //     println!("В {}-й сезон урожай составит {} зерен", i, result)
    // }

    const YEARS: i32 = 20;
    const CREDIT: i32 = 2000000;
    const PERCENT: i32 = 12;
    let mut over = 0;

    for i in 0..=YEARS {
        over = over + CREDIT * PERCENT / 100;
        let result = over + CREDIT;
        //println!("{result}");
    }

    let mortgage: i32 = 2000000; // ипотека, рублей
    let period: i32 = 240; // месяцы
    let bank_interest: i32 = 12; // процент годовых
    let mut mort_fee: i32 = 0; // плата за кредит, рублей
    let mut mort_balanse: i32 = mortgage; // остаток по ипотеке, рублей

    for _ in 1..=period {
        mort_balanse -= mortgage / 240;
        mort_fee += mort_balanse * bank_interest / 12 / 100;
        println!("{mort_fee}")
    }
    println!(
       "За кредит {}руб. сроком {} месяцев под {}% годовых будет отдано {} тяжко заработанных рублей.",
       mortgage, period, bank_interest, mort_fee
   );

    let mut semen = 937;
    let mut sum = 0;
    for i in 1..=semen {
        if i % 5 == 0 {
            continue;
        };
        sum += 1;
    }
    println!("{}", sum);

    let start_point: (f32, f32) = (2.0, 2.0);
    let end_point: (f32, f32) = (4.0, 4.0);
    println!("x1:{}, y1:{}", start_point.0, start_point.1);
    println!("x2:{}, y2:{}", end_point.0, end_point.1);
    let length: f32 =
        ((start_point.0 - end_point.0).powf(2.0) + (start_point.1 - end_point.1).powf(2.0)).sqrt();
    println!("Длина линии {}", length);

    let start_point: (f32, f32) = (2.0, 2.0);
    let end_point: (f32, f32) = (start_point.0 * 2.0, start_point.1 * 3.0);
    println!("{:?}{:?}", start_point, end_point);

    let grades: [u8; 4] = [2, 7, 4, 5];
    let mut temp = 0;
    for p in grades {
        if temp < p {
            temp = p;
        }
    }
    println!("{temp}");

    let mut shopping: [&str; 5] = ["Samara", "Moscow", "London", "Saratov", "Ekaterinburg"];
    for p in shopping {
        if p == "Saratov" {
            print!("{p}");
            break;
        }
    }

    let arr: [i32; 10] = [0, -14, 62, 5, -5, 3, 20, 0, 88, -17];
    let len = arr.len();
    let mut sum = 0;
    for i in (len / 2..len) {
        if arr[i] > 0 {
            sum += arr[i];
        }
    }
    println!("summa: {}", sum);

    let mut arr: [i32; 10] = [4, 75, 9, -89, -1, 40, 0, 0, 94, 1];
    let len = arr.len();
    let mut temp: i32;
    println!("Массив до сортировки:    {:?}", arr);
    for i in 0..(len - 1) {
        for j in i..len {
            if arr[i] > arr[j] {
                temp = arr[i];
                arr[i] = arr[j];
                arr[j] = temp;
            }
        }
    }
    println!("Массив после сортировки: {:?}", arr);

    let mut names = vec!["bread", "milk", "cheese", "apple", "noodles"];
    let guest = "Victor";

    if names.contains(&guest) {
        print!("V spiske, GO!");
    } else {
        print!("no, go away!");
        names.push(guest)
    }
    println!("{:?}", names);

    let before = "идис ен ,янес иди".to_string();
    for s in before.chars().rev() {
        println!("{}", s)
    }

    fn hello() {
        println!("Hi, have a nice day!");
    }
    hello();

    fn wich_month(x: i32) {
        match x {
            1 => println!("jan"),
            2 => println!("feb"),
            3 => println!("mar"),
            4 => println!("apr"),
            5 => println!("may"),
            6 => println!("jun"),
            7 => println!("jul"),
            8 => println!("aug"),
            9 => println!("sep"),
            10 => println!("oct"),
            _ => println!("no"),
        };
    }

    wich_month(55);
}
