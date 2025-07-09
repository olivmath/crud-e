use crate::state::AppState;
use tide::Request;

pub async fn delete_data(req: Request<AppState>) -> tide::Result {
    // Extrai o id da URL (ex: /data/:id)
    let id: u32 = match req.param("id")?.parse() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("❌ Erro: ID inválido na requisição DELETE /data/:id");
            return Err(tide::Error::from_str(400, "Invalid id"));
        }
    };

    // Pega o estado global
    let state = req.state();
    let mut map = state.lock().unwrap();

    // Remove o registro se existir
    if map.remove(&id).is_some() {
        println!("✅ Registro deletado com sucesso (ID: {})", id);
        Ok(tide::Response::new(204))
    } else {
        println!("❌ Registro não encontrado para deleção (ID: {})", id);
        Ok(tide::Response::new(404))
    }
}
