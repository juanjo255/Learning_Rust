use core::str;
use std::any::Any;

fn main() {
    println!("-----------------------------------------------");
    println!("AQUI VIENE EL TEMA DE VARIABLES Y CONSTANTES");
    
    // Variables
    let inmutable = "This is an inmutable variable";
    let mut mutable = "this is a mutable variable";
    println!("variable outter scope: {inmutable}");
    println!("variable mutable: {mutable} ");

    {
        // Shadowing and scope
        let inmutable = "inmutable variable changed at inner scope";
        println!("variable inner scope: {inmutable}")
    }
    println!("variable outter scope: {inmutable}");
    
    mutable = "changing mutable variable";
    println!("variable mutable: {mutable} ");

    // Constants
    const CONSTANT_NUMBER:u32 = 2 * 10;
    println!("constant inner scope: {CONSTANT_NUMBER}");

    println!("-----------------------------------------------");
    struct_fun();
    println!("-----------------------------------------------");
    enums_fun();
    println!("-----------------------------------------------");
    enums_with_match();

}

fn struct_fun (){
    println!("AQUI VIENE EL TEMA DE ENUMS Y STRUCT");

    // DEFINIENDO UNA STRUCT
    struct User {
        username: &'static str,
        email: String,
        active: bool
    }

    // INSTANCIANDO UNA STRUCT

    let mut user1 = User {
        email: String::from("piconcossio@gmail.com"),
        username: "juanjo255",
        active: true
    };

    // ACCEDER Y CAMBIAR UN FIELD
    user1.username="rosamelano";
    
    // UNA FUNCION QUE RETORNA UNA INSTANCIA DE UNA STRUCT
    fn build_user(email: String, username:  &'static str) -> User {
        User {
            email, //: email,
            username, //: username,
            active: true,
        }
    }
    let user2 = build_user(String::from("luchoportuano@gmail.com"), "F");

    // CREAR UNA INSTANCIA A PARTIR DE OTRA
    // PERO ESTA INSTANCIA REEMPLAZA A LA ANTERIOR EN ESTE CASO POR EL STRING TYPE
    // SE MUEVE EL OWNERSHIP

    //let user3 = User {
    //    active: user1.active,
    //    username: user1.username,
    //    email: String::from("another@example.com")
    //};
        // Esto es lo mismo que hacer lo de arriba, pero con menos codigo
    let user3 = User {
        email: String::from("cambiandoSoloEmail"),
        ..user1
    };

    // TUPLE STRUCT 
    struct Fecha(i32, &'static str, &'static str);
    let fecha_actual = Fecha(13, "agosto", "2022");
    // ACCEDER Y DESTRUCTURAR UNA TUPLE STRUCT
    let Fecha (a, b, c) = fecha_actual;
    println!("accediendo al index 1 de una tuple struct: {}", fecha_actual.1);


}

fn enums_fun (){
    println!("AQUI VIENE ENUMS O ENUMERATE");

    // DEFINIMOS LAS OPCIONES DEL ENUM
    #[derive(Debug)]
    enum IpAddr {
        V4 (u8, u8, u8, String),
        V6 (String)
    }

    // PODEMOS CREAR INSTANCES DE LAS OPCIONES DEL ENUM
    let four = IpAddr::V4(127, 0, 0, String::from("0"));
    let six = IpAddr::V6(String::from("::1"));

    println!("MI ENUM CON SU TYPE ASOCIADO: {:?}", four);

}

fn enums_with_match (){
    println!("AQUI SE EXPONE MATCH EN COMBINACION CON ENUMS");
    let _a:Option<i32> = None;

    match _a {
        None => println!("severo none {:?}",_a),
        Some(_i) => print!("F"),
    }
    // CREAMOS UN ENUM
    enum Coin {
        Penny,
        Nickel
    }

    // MATCH QUE DEPENDIENDO DE LA MONEDA DEVUELVE SU VALOR NUMERICO
    fn value_in_cents (coin:Coin) -> u8{
        match coin {
            Coin::Nickel => {
                println!("THIS IS A CODE WITH A MATCH, VALUE OF COIN {}", 5);
                5},
            Coin::Penny => 1
        }
    }
}
