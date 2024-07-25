use clap::{Parser, ValueEnum};

use qdrant_client::qdrant::{CreateCollectionBuilder, Distance, VectorParamsBuilder};
use qdrant_client::Qdrant;

use serde::Serialize;

#[derive(Clone, Debug, Serialize, ValueEnum)]
enum DbHost {
    Local,
    Docker,
}

#[derive(Debug, Parser, Serialize)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long, value_enum)]
    db_host: DbHost,
}

impl DbHost {
    #[allow(dead_code)]
    fn to_possible_value(&self) -> Option<&str> {
        match self {
            DbHost::Local => Some("local"),
            DbHost::Docker => Some("docker"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    println!("{:?}", args);

    let db_uri = match args.db_host {
        DbHost::Local => "http://localhost:6334",
        DbHost::Docker => "http://qdrant:6334",
    };

    let client = Qdrant::from_url(db_uri).build()?;

    let collection_name = "lots_final_images_collection";
    let response = client.collection_exists(collection_name).await?;
    if !response {
        println!("Collection {collection_name} does not exist. Creating...");
        client
            .create_collection(
                CreateCollectionBuilder::new(collection_name)
                    .vectors_config(VectorParamsBuilder::new(512, Distance::Cosine)),
            )
        .await?;
    } else {
        println!("Collection {collection_name} exists!");
    }

    Ok(())
}
