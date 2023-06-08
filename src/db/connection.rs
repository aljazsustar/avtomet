use mongodb::{Client, options::{ClientOptions, ServerApi, ServerApiVersion}, bson::Document};

pub struct Connection {
    pub host: String,
    pub port: String,
    pub db: String,
    pub client: Client
}

impl Connection {
    pub async fn new(host: String, port: String, db: String) -> Self {
        let mut client_options = ClientOptions::parse(format!("{}:{}", host, port)).await.expect("Unable to parse connection URI");
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        let client = Client::with_options(client_options).unwrap();
        client.database(&db);
        
        Self {host, port, db, client}
    }

    pub async fn run_command(&self, command: Document) -> Document {
        self.client.database(&self.db).run_command(command, None).await.unwrap()
    }
}