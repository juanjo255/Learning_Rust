
fn main() {
    
    // CLOSURES

    // Can a closure be a function using impl?
    pub struct LivingBeing {
        cells:&'static str,
        cellular_org:String,
        specie:String
    }

    let _human = LivingBeing{
        cells:"Eukariote",
        cellular_org: String::from("Pluricellular"),
        specie:String::from("Homo sapiens")
    };
}
