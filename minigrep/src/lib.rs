use std::fs;
use std::error::Error;
// Este modulo nos permite revisar la enviroment variables
use std::env;

// Se utiliza esta estructura que nos permite tener organizacion en el codigo
// Le dice al programador "Hey estos 2 datos se manejan juntos"
pub struct  Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
// Ya que tenemos la estructura que almacena nuestros datos de interes
// Se implementa un metodo que inicialize dicha estructura
// De entrada notese el impl Iterator<Item=String>, este indica que el argumento sera cualquier tipo que implemente dicho Trait
// Ese trait lo necesitamos para iterar sobre los argumentos dados por el shell
// Se utiliza de salida un Result que nos indica de si se lo logro la creacion de la estructura o hubo un error
// De modo que en la contruccion del codigo principal (main) se pueda dar una respuesta en caso de un error
impl Config {
    pub fn build(mut args: impl Iterator<Item = String> ) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        return Ok(Config { query, file_path, ignore_case})
    }
}

// Funcion principal: Corre la funcion search de la herramienta
// Aqui tambien se usa Result con el objetivo de manejar el error, pero ya que solo 
//  nos interesa imprimir en patalla, en caso de una corrida exitosa no hay un tipo de dato solo un print,
// Sin embargo en caso de erros usamos Box, que nos permite manejar diferentes tipos de error
// Aun no se como MONDA funciona Box ni el signo de interrogacion :v
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    return Ok(())
}
// Esta Funcion que nos premite buscar la palabra de interes en una string
// Notese el <'a> es el lifetime que conecta el pointer de contents al vector saliente
// Ya que este vector saliente va a poseer slices del contents y no del query, entonces
// hay que dejarle claro eso a RUST
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// Funcion adiccional que permite busquedas sin importar si hay mayusculas
// Se hace para entender las enviroment variables y como estas sirven para darle opciones al usuario
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str,) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

// Prueba unitaria
// Aqui simplemente se evalua que la funcion search esta funcionando
// Notese los # - importantes para que al correr cargo test 
// Rust sepa que debe compilar y correr solo estas funciones 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duck tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."], search_case_insensitive(query, contents)
        );
    }
}