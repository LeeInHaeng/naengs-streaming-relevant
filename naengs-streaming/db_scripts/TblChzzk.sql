------------------------
CREATE TABLE TblChzzkLive
(
	 _liveId					INTEGER			NOT NULL		-- 라이브 식별자
	,_liveTitle					VARCHAR(200)	NOT NULL		-- 라이브 제목
	,_liveThumbnailImageUrl		VARCHAR(500)		NULL		-- 라이브 썸네일로 사용되는 이미지 URL
	,_concurrentUserCount		INT				NOT NULL		-- 라이브 현재 시청자 수
	,_openDate 					TIMESTAMP		NOT NULL		-- 라이브 시작 시간
	,_adult						BOOLEAN			NOT NULL		-- 연령 제한 설정 여부
	,_tags						VARCHAR(200)	NOT NULL		-- 라이브에 설정된 태그 목록 , 구분
	,_categoryType				VARCHAR(50)		NOT NULL		-- 카테고리 종류
	,_liveCategory				VARCHAR(50)		NOT NULL		-- 라이브 카테고리 식별자
	,_liveCategoryValue			VARCHAR(50)		NOT NULL		-- 라이브 카테고리 이름
	,_channelId					VARCHAR(100)	NOT NULL		-- 채널 ID(채널 식별자)
	,_channelName				VARCHAR(100)	NOT NULL		-- 채널명
	,_channelImageUrl			VARCHAR(500)	NOT NULL		-- 채널 이미지 URL
);