pub mod consulta {

    use reqwest;
    use serde_json::Value;

    const API_NOME: &str = "http://api3.beagafans.site:8080/api/nome?nome=";
    const API_CPF: &str = "http://api3.beagafans.site:8080/api/informacoes?cpf=";
    const API_TEL1: &str = "http://api3.beagafans.site:8080/api/spctel?telefone=";
    const API_TEL2: &str = "http://api3.beagafans.site:8080/api/telefoni?telefone=";
    const API_TEL3: &str =
        "http://api3.beagafans.site:8080/api/historico-telefone-completo?telefone=";

    pub async fn get(api: &str, dados: &String) -> Result<String, reqwest::Error> {
        let req: String = reqwest::get(format!("{}{}", api, &dados))
            .await?
            .text()
            .await?;
        Ok(req)
    }

    pub async fn nome(args: Vec<String>) -> Result<(), reqwest::Error> {
        let mut nome = String::new();
        for arg in 1..args.len() {
            nome.push_str(&args[arg]);
            if arg != args.len() - 1 {
                nome.push_str("%20");
            }
        }
        let nome = get(API_NOME, &nome.to_uppercase()).await?;
        let v: Value = serde_json::from_str(&nome).expect("Error read json");
        println!("{}", v);
        Ok(())
    }
}
