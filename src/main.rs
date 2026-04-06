mod product;
mod index;
mod graph;

use product::Product;
use index::SearchIndex;
use graph::ProductGraph;

fn main() {
    let mut search_engine = SearchIndex::new();
    let mut recommender = ProductGraph::new();

    let p1 = Product::new(1, "Smartphone Samsung Galaxy", "Eletrônicos", 2500.0);
    let p2 = Product::new(2, "Smartphone Apple iPhone", "Eletrônicos", 5500.0);
    let p3 = Product::new(3, "Fone Bluetooth Noise Cancelling", "Acessórios", 800.0);

    search_engine.add_product(p1.clone());
    search_engine.add_product(p2.clone());
    search_engine.add_product(p3.clone());

    recommender.connect(1, 3);
    recommender.connect(2, 3);

    println!("--- MEGATORE SEARCH & RECOMMENDATION ---");

    
    let query = "smartphone";
    println!("\n🔍 Buscando por: '{}'", query);
    let results = search_engine.search(query);
    
    for p in &results {
        println!("Encontrado: {} - R${:.2}", p.name, p.price);
        
        let recs = recommender.get_recommendations(p.id);
        if !recs.is_empty() {
            print!("   💡 Recomendação (Grafo): ");
            for rec_id in recs {
                if let Some(prod) = search_engine.registry.get(&rec_id) {
                    print!("[{}] ", prod.name);
                }
            }
            println!();
        }
    }
}