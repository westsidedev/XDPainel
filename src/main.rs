use std::env;
use XDPainel::consulta;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args[0].as_str() {
        "-n" | "--nome" => {
            consulta::nome(args).await?;
        }
        "-c" | "--cpf" => {
            //consulta::post(&args[1]).await?;
            ()
        }
        &_ => (),
    }
    Ok(())
}
