use clap::{Parser, ValueEnum};

use qdrant_client::qdrant::{
    Condition,
    CreateCollectionBuilder, Distance, Filter, PointStruct, SearchPointsBuilder, UpsertPointsBuilder,
    VectorParamsBuilder,
};
use qdrant_client::{Payload, Qdrant};

use serde::Serialize;
use serde_json::json;

#[derive(Clone, Debug, Serialize, ValueEnum)]
enum DbHost {
    Local,
    Docker,
}

#[derive(Debug, Parser, Serialize)]
#[command(version, about, long_about = None)]
struct Args {
    /// Select the DB host, depending on whether to use the DB
    /// from a docker container or the host machine
    #[arg(long, value_enum)]
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
    let ndims: usize = 768;

    let args = Args::parse();

    let db_uri = match args.db_host {
        DbHost::Local => "http://localhost:6334",
        DbHost::Docker => "http://qdrant:6334",
    };

    let client = Qdrant::from_url(db_uri).build()?;

    let collection_name = "image_embeddings";
    let response = client.collection_exists(collection_name).await?;
    if !response {
        println!("Collection {collection_name} does not exist. Creating...");
        client
            .create_collection(
                CreateCollectionBuilder::new(collection_name)
                    .vectors_config(VectorParamsBuilder::new(ndims as u64, Distance::Cosine)),
            )
        .await?;
    } else {
        println!("Collection {collection_name} exists!");
    }

    let points = vec![
        PointStruct::new(
            1,
            vec![1.0; ndims],
            Payload::try_from(json!(
                {"item_id": "item001", "make": "Jaguar", "model": "F-Type", "year": 2015}
            ))
                .unwrap(),
        ),
        PointStruct::new(
            2,
            vec![1.1; ndims],
            Payload::try_from(json!(
            {"item_id": "item002", "make": "Ford", "model": "GT-40", "year": 1972}
            ))
                .unwrap(),
        ),
        PointStruct::new(
            3,
            vec![1.1; ndims],
            Payload::try_from(json!(
            {"item_id": "item003", "make": "Jaguar", "model": "E-Type", "year": 1969}
            ))
                .unwrap(),
        ),
    ];

    let response = client
        .upsert_points(UpsertPointsBuilder::new(collection_name, points).wait(true))
    .await?;

    dbg!(response);

    let search_result = client
        .search_points(
            SearchPointsBuilder::new(collection_name, vec![1.2; ndims], 3).filter(Filter::all([Condition::matches(
                "model",
                "E-Type".to_string(),
            )])).with_payload(true),
        )
    .await?;

    dbg!(search_result);

    Ok(())
}
