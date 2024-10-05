// first one
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MangaSearch {
    pub data: Vec<MangaData>,
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MangaData {
    pub id: String,
    pub attributes: Option<MangaAttribute>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MangaAttribute {
    pub title: Option<MangaTitle>,
    pub description: Option<MangaDescription>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MangaDescription {
    pub en: Option<String>,
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MangaTitle {
    pub en: Option<String>,
}

// 2nd one
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ChapterInfo {
    pub data: Vec<Chapter>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Chapter {
    pub id: String,
    pub attributes: ChapterAttribute,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ChapterAttribute {
    pub volume: Option<String>,
    pub chapter: Option<String>,
    pub title: Option<String>,
}



#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageResult {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub chapter: ImageChapter,
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ImageChapter {
    pub hash: String,
    pub data: Vec<String>,
    #[serde(rename = "dataSaver")]
    pub data_saver: Vec<String>,
}
