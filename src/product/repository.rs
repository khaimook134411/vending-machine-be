use bson::oid::ObjectId;
use crate::config::database::db_connect;
use crate::product::entity::InsertProductRequest;

pub async fn insert_one_product(req: InsertProductRequest)-> Result<ObjectId, String>{
  let client =  match db_connect().await {
        Ok(mut client) => client,
        Err(e) => panic!("Error: Database connection fialed:  {:?}",e)
    };



    Ok(ObjectId::new() )
}