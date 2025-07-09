mod handlers;
mod models;
mod state;

use handlers::create::create_data;
use handlers::delete::delete_data;
use handlers::read::{read_all_data, read_data};
use handlers::update::update_data;

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("🚀 Iniciando servidor CRUD...");
    
    // Cria o estado global da aplicação
    let state = state::new_state();
    println!("✅ Estado da aplicação criado");

    // Cria o app Tide e associa o estado
    let mut app = tide::with_state(state);
    println!("✅ Aplicação Tide configurada");

    // Define as rotas CRUD
    app.at("/data").post(create_data); // Cria
    app.at("/data").get(read_all_data); // Lê todos
    app.at("/data/:id").get(read_data); // Lê um
    app.at("/data/:id").put(update_data); // Atualiza
    app.at("/data/:id").delete(delete_data); // Deleta
    println!("✅ Rotas CRUD configuradas:");

    let addr = "0.0.0.0:8080";
    println!("🌐 Servidor CRUD rodando em: http://{addr}");
    println!("📋 Endpoints disponíveis:");
    println!("   POST   /data     - Criar novo registro");
    println!("   GET    /data     - Listar todos os registros");
    println!("   GET    /data/:id - Buscar registro por ID");
    println!("   PUT    /data/:id - Atualizar registro");
    println!("   DELETE /data/:id - Deletar registro");

    // Inicia o servidor
    app.listen(addr).await?;
    Ok(())
}
