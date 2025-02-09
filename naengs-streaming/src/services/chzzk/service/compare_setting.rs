use tracing::warn;

use crate::{services::chzzk::{db_models::compare_setting_model::InitLiveSelectResult, models::common::CommonResult}, DB_POOL_STREAMING};

pub async fn init() -> CommonResult {
    let mut result = CommonResult::default();

     let live_select_query = String::from("
     SELECT  _liveId
            ,_liveTitle
            ,_liveThumbnailImageUrl
            ,_concurrentUserCount
            ,_openDate
            ,_adult
            ,_tags
            ,_categoryType
            ,_liveCategory
            ,_liveCategoryValue
            ,_channelId
            ,_channelName
            ,_channelImageUrl
        FROM TblChzzkLive");
 
     let client = match DB_POOL_STREAMING.get().await {
         Ok(res) => res,
         Err(err) => {
             warn!("init_compare_setting :: get DB Pool client err : {}", err);
             result._result_code = -1;
             result._result_msg = "init_compare_setting :: get DB Pool client err".to_string();
             return result;
         }
     };
 
     let stmt = match client.prepare(&live_select_query).await {
         Ok(res) => res,
         Err(err) => {
             warn!("init_compare_setting :: query err : {}", err);
             result._result_code = -2;
             result._result_msg = "init_compare_setting :: query err".to_string();
             return result;
         }
     };
 
     let select_result = client.query(&stmt, &[]).await;
 
     let live_list: Vec<InitLiveSelectResult> = match select_result {
         Ok(rows) => {
             rows
             .iter()
             .map(|row| {
                InitLiveSelectResult {
                    _liveId: row.get(0),
                    _liveTitle: row.get(1),
                    _liveThumbnailImageUrl: row.get(2),
                    _concurrentUserCount: row.get(3),
                    _openDate: row.get(4),
                    _adult: row.get(5),
                    _tags: row.get(6),
                    _categoryType: row.get(7),
                    _liveCategory: row.get(8),
                    _liveCategoryValue: row.get(9),
                    _channelId: row.get(10),
                    _channelName: row.get(11),
                    _channelImageUrl: row.get(12),
                 }
             })
             .collect()
         },
         Err(err) => {
             warn!("init_compare_setting :: live_list err: {}",err);
             vec![]
         }
     };
 
     result
}