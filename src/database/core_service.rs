use super::mongodb::MongoClient;
use crate::{handlers::error_handler::Errors, traits::model::ModelTrait};
use chrono::Utc;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    options::{
        AggregateOptions, FindOneAndDeleteOptions, FindOneAndUpdateOptions, FindOneOptions,
        InsertOneOptions, UpdateModifications,
    },
    results::InsertOneResult,
    ClientSession,
};
use serde::{de::DeserializeOwned, Serialize};

impl MongoClient {
    pub async fn create_one<M: DeserializeOwned + Serialize + Clone + ModelTrait>(
        &self,
        collection_name: impl Into<String>,
        mut model: M,
        options: Option<InsertOneOptions>,
        session: Option<ClientSession>,
    ) -> Result<InsertOneResult, Errors> {
        let id = ObjectId::new().to_string();
        model.set_id(id);
        let current_timestamp = Utc::now().timestamp() as u64;
        model.set_created_at(current_timestamp);
        model.set_updated_at(current_timestamp);
        let collection = self
            .client
            .database(&self.db_name)
            .collection::<M>(collection_name.into().as_str());
        let result = match session {
            None => collection.insert_one(model, options).await,
            Some(mut mongo_session) => {
                collection
                    .insert_one_with_session(model, options, &mut mongo_session)
                    .await
            }
        };
        match result {
            Ok(insert_result) => Ok(insert_result),
            Err(error) => Err(Errors::InternalError(error.to_string())),
        }
    }
    pub async fn read_one<Model: DeserializeOwned + Serialize + Clone + Sync + Send + Unpin>(
        &self,
        collection_name: String,
        data_filter: Document,
        options: Option<FindOneOptions>,
    ) -> Result<Option<Model>, Errors> {
        let collection = self
            .client
            .database(&self.db_name)
            .collection::<Model>(collection_name.as_str());
        let result = collection
            .find_one(data_filter, options)
            .await
            .map_err(|error| Errors::InternalError(error.to_string()))?;
        Ok(result)
    }

    pub async fn update_one<Model: DeserializeOwned>(
        &self,
        collection_name: String,
        data_filter: Document,
        update: UpdateModifications,
        options: Option<FindOneAndUpdateOptions>,
        session: Option<ClientSession>,
    ) -> Result<Option<Model>, Errors> {
        let collection = self
            .client
            .database(&self.db_name)
            .collection::<Model>(collection_name.as_str());
        let current_timestamp = Utc::now().timestamp() as u32;
        let update_doc = doc! {"$set": {"updated_at": current_timestamp}};
        let new_update = match update {
            UpdateModifications::Document(doc) => {
                UpdateModifications::Pipeline(vec![doc, update_doc])
            }
            UpdateModifications::Pipeline(pipeline) => {
                let mut pipeline_vec = pipeline;
                pipeline_vec.push(update_doc);
                UpdateModifications::Pipeline(pipeline_vec)
            }
            _ => return Err(Errors::InternalError("Pipeline error".to_string())),
        };
        let result = match session {
            Some(mut session) => {
                collection
                    .find_one_and_update_with_session(
                        data_filter,
                        new_update,
                        options,
                        &mut session,
                    )
                    .await
            }
            None => {
                collection
                    .find_one_and_update(data_filter, new_update, options)
                    .await
            }
        };
        match result {
            Ok(update_result) => Ok(update_result),
            Err(error) => Err(Errors::InternalError(error.to_string())),
        }
    }

    pub async fn delete_one<Model>(
        &self,
        collection_name: String,
        data_filter: Document,
        options: Option<FindOneAndDeleteOptions>,
        session: Option<ClientSession>,
    ) -> Result<Option<Model>, Errors>
    where
        Model: DeserializeOwned + Serialize + Clone + ModelTrait,
    {
        let collection = self
            .client
            .database(&self.db_name)
            .collection::<Model>(collection_name.as_str());
        let result = match session {
            Some(mut session) => {
                collection
                    .find_one_and_delete_with_session(data_filter, options, &mut session)
                    .await
            }
            None => collection.find_one_and_delete(data_filter, options).await,
        };

        match result {
            Ok(delete_result) => Ok(delete_result),
            Err(error) => Err(Errors::InternalError(error.to_string())),
        }
    }
    pub async fn query_read<Model>(
        &self,
        collection_name: String,
        aggregate: Vec<Document>,
        page: Option<u8>,
        page_size: Option<u8>,
        paging_data: bool,
        options: Option<AggregateOptions>,
    ) -> Result<Document, Errors> {
        let collection = self
            .client
            .database(&self.db_name)
            .collection::<Model>(collection_name.as_str());
        let mut aggregate_pipeline = aggregate.clone();
        if paging_data {
            let page_size: i32 = match page_size {
                Some(val) => {
                    if val >= 100 {
                        10
                    } else {
                        val.into()
                    }
                }
                None => 10,
            };
            let page: i32 = match page {
                Some(val) => val.into(),
                None => 1,
            };
            let skip: i32 = (page - 1) * page_size;

            let mut additional_aggregate = vec![
                doc! {"$facet": {"data": [{"$skip": skip}, {"$limit": page_size + 1}], "total_count": [{"$count": "total"}]}},
                doc! {
                    "$addFields": {
                        "metadata": {
                            "current_page": page,
                            "page_size": page_size,
                            "total_records": {"$ifNull": [{"$arrayElemAt": ["$total_count.total", 0]}, 0]},
                            "has_next_page": {"$gt": [{"$size": "$data"}, page_size]},
                        }
                    }
                },
                doc! { "$project": {"data": {"$slice": ["$data", page_size]}, "metadata": 1} },
            ];
            aggregate_pipeline.append(&mut additional_aggregate);
        }
        let mut cursor = collection
            .aggregate(aggregate_pipeline, options)
            .await
            .map_err(|error| Errors::InternalError(error.to_string()))?;
        let mut result_document = Document::new();
        while cursor
            .advance()
            .await
            .map_err(|error| Errors::InternalError(error.to_string()))?
        {
            let doc = cursor
                .deserialize_current()
                .map_err(|error| Errors::InternalError(error.to_string()))?;
            result_document.extend(doc);
        }
        /*
                while let Some(doc) = cursor.next().await {
                    match doc {
                        Ok(item) => result_document.extend(item),
                        Err(error) => return Err(Errors::InternalError(error.to_string())),
                    }
                }
        */
        Ok(result_document)
    }
}
