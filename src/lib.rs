use std::fs;
use std::error::Error;

// Se utiliza esta estructura que nos permite tener organizacion en el codigo
// Le dice al programador "Hey estos 2 datos se manejan juntos"
pub struct  Config {
    pub query: String,
    pub file_path: String,
}
// Ya que tenemos la estructura que almacena nuestros datos de interes
// Se implementa un metodo que inicialize dicha estructura
// Se utiliza de salida un Result que nos indica de si se lo logro la creacion de la estructura o hubo un error
// De modo que en la contruccion del codigo principal (main) se pueda dar una respuesta en caso de un error
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        return Ok(Config { query, file_path })
    }
}

// Funcion principal: Corre la funcion search de la herramienta
// Aqui tambien se usa Result con el objetivo de manejar el error, pero ya que solo 
//  nos interesa imprimir en patalla, en caso de una corrida exitosa no hay un tipo de dato solo un print,
// Sin embargo en caso de erros usamos Box, que nos permite manejar diferentes tipos de error
// Aun no se como MONDA funciona Box ni el signo de interrogacion :v
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    return Ok(())
}
// Esta Funcion que nos premite buscar la palabra de interes en una string
// Notese el <'a> es el lifetime que conecta el pointer de contents al vector saliente
// Ya que este vector saliente va a poseer slices del contents y no del query, entonces
// hay que dejarle claro eso a RUST
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    return results
}

// Prueba unitaria
// Aqui simplemente se evalua que la funcion search esta funcionando
// Notese los # - importantes para que al correr cargo test 
// Rust sepa que debe compilar y correr solo estas funciones 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}