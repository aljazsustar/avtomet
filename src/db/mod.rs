use mongodb::{Collection, Cursor, bson::{doc, Document}, options::FindOptions};

use crate::parser::oglas::Oglas;

use self::connection::Connection;

pub mod connection;

pub struct MongoDB {
    connection: Connection,
    pub ads_collection: Collection<Oglas>
}

impl MongoDB {
    pub async fn init(host: String, port: String, db: String) -> Self {
        let connection = Connection::new(host, port, db.clone()).await;
        let ads_collection = connection.client.database(&db).collection::<Oglas>("ads");
        Self { connection, ads_collection }
    }

    pub async fn insert_ad(&self, oglas: Oglas) -> () {
        self.ads_collection.insert_one(oglas, None).await.expect("Insertion failed");
    }

    pub async fn insert_ads(&self, oglasi: &Vec<Oglas>) -> () {
        self.ads_collection.insert_many(oglasi, None).await.expect("Insertion failed");
    }

    pub async fn fetch_all_ads(&self) -> Cursor<Oglas> {
        self.ads_collection.find(None, None).await.expect("Fetch all ads failed")
    }

    pub async fn filter_ads_by_znamka(&self, znamka: String) -> Cursor<Oglas> {
        self.ads_collection.find(doc! {"znamka": znamka}, None).await.expect("Fetch with filter by znamka failed")
    }

    pub async fn filter_ads(&self, filter: Document) -> Cursor<Oglas> {
        self.ads_collection.find(filter, None).await.expect("Fetch ads with filter failed")
    }

    pub async fn order_ads(&self, options: FindOptions) -> Cursor<Oglas> {
        self.ads_collection.find(None, options).await.expect("Fetch with order failed")
    }

    pub async fn fetch_order_ads(&self, filter: Document, options: FindOptions) -> Cursor<Oglas> {
        self.ads_collection.find(filter, options).await.expect("Fetch wirth filter and order failed")
    }
}