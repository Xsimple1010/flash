use std::fs;

use crate::state::Executable;

use reqwest::multipart;

pub enum ExeType {
    Main,
    Dep,
}

pub async fn send_executable(
    base_url: String,
    exe_type: ExeType,
    exe: &Executable,
) -> Result<(), Box<dyn std::error::Error>> {
    // Abre o arquivo
    let file = fs::read(exe.path.clone())?;
    // Cria a parte do arquivo para o formulário multipart
    let part = multipart::Part::stream(file)
        .file_name(exe.name.clone())
        .mime_str("text/plain")?;

    // Cria o formulário multipart
    let form = multipart::Form::new().part(exe.name.clone(), part);

    // Cria o cliente reqwest
    let client = reqwest::Client::new();

    let url = match exe_type {
        ExeType::Main => format!("{}/exe", base_url),
        ExeType::Dep => format!("{}/dep", base_url),
    };

    // Envia a requisição POST com o formulário
    let response = client.post(url).multipart(form).send().await?;

    // Verifica o status da resposta
    if response.status().is_success() {
        println!("Arquivo enviado com sucesso!");
    } else {
        println!("Erro ao enviar o arquivo: {:?}", response.status());
    }

    Ok(())
}
