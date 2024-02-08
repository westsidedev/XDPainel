use reqwest;
use std::env;
use std::io;
use std::io::Write;

const API_NOME: &str = "http://api3.beagafans.site:8080/api/nome?nome=";
const API_CPF: &str = "http://api3.beagafans.site:8080/api/informacoes?cpf=";

struct Consulta(String);

impl Consulta {
    async fn nome(&self) -> Result<(), reqwest::Error> {
        let request = reqwest::get(format!("{}{}", API_NOME, &self.0.to_uppercase()))
            .await?
            .text()
            .await?;
        println!("{:?}", request);
        Ok(())
    }

    async fn cpf(&self) -> Result<(), reqwest::Error> {
        let request = reqwest::get(format!("{}{}", API_CPF, &self.0.to_uppercase()))
            .await?
            .text()
            .await?;
        println!("{:?}", request);
        Ok(())
    }
}

async fn menu() -> String {
    print!("1 - NOME\n2 - CPF\n>>");
    io::stdout().flush().unwrap();
    let mut opc = String::new();
    io::stdin().read_line(&mut opc).expect("Error");
    //let opc: i32 = opc.trim().parse().expect("Error ao ler a entrada");
    opc.trim().to_string()
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let opc = menu().await;
    match opc.as_str() {
        "1" => {
            print!("Digite o nome:");
            io::stdout().flush().unwrap();
            let mut nome = String::new();
            io::stdin()
                .read_line(&mut nome)
                .expect("Error ao ler o nome");
            let nome: Consulta = Consulta(nome.replace(" ", "%20"));
            Consulta::nome(&nome).await?;
        }
        "2" => {
            println!("Digite o cpf:");
            let mut cpf = String::new();
            io::stdin().read_line(&mut cpf).expect("Error ao ler cpf");
            let cpf: Consulta = Consulta(cpf);
            Consulta::cpf(&cpf).await?;
        }
        &_ => (),
    }
    Ok(())
}
