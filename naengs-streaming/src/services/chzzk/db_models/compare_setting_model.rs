use chrono::NaiveDateTime;

#[allow(non_snake_case)]
pub struct InitLiveSelectResult {
    pub _liveId: i32,
    pub _liveTitle: String,
    pub _liveThumbnailImageUrl: String,
    pub _concurrentUserCount: i32,
    pub _openDate: NaiveDateTime,
    pub _adult: bool,
    pub _tags: String,
    pub _categoryType: String,
    pub _liveCategory: String,
    pub _liveCategoryValue: String,
    pub _channelId: String,
    pub _channelName: String,
    pub _channelImageUrl: String,
}