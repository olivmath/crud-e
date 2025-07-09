use crate::state::AppState;
use tide::Request;

pub async fn read_all_data(req: Request<AppState>) -> tide::Result {
    // Pega o estado global
    let state = req.state();
    let map = state.lock().unwrap();

    let count = map.len();
    println!("📊 Total de registros encontrados: {}", count);

    // Retorna todos os registros como JSON
    Ok(tide::Body::from_json(&*map)?.into())
}

pub async fn read_data(req: Request<AppState>) -> tide::Result {
    // Extrai o id da URL (ex: /data/:id)
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("❌ Erro: ID inválido na requisição GET /data/:id");
            return Err(tide::Error::from_str(400, "Invalid id"));
        }
    };

    // Pega o estado global
    let state = req.state();
    let map = state.lock().unwrap();

    // Procura o registro pelo id
    if let Some(entry) = map.get(&id) {
        println!("✅ Registro encontrado (ID: {})", id);
        Ok(tide::Body::from_json(entry)?.into())
    } else {
        println!("❌ Registro não encontrado para atualização (ID: {})", id);
        Ok(tide::Response::new(404))
    }
}
