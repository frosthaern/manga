#[allow(unused_imports)]
use std::io::Write;
use tokio::io::AsyncWriteExt;

#[allow(dead_code)]
pub async fn make_api_request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let headers = crate::headers::headers().await;
    let res = client.get(url).headers(headers).send().await?;

    match res.status() {
        reqwest::StatusCode::OK => Ok(res.text().await?),
        _ => Err("status code not ok".into()),
    }
}

#[allow(dead_code)]
pub async fn get_image_bytes(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let header = crate::headers::headers().await;
    let res = client.get(url).headers(header).send().await?;
    match res.status() {
        reqwest::StatusCode::OK => Ok(res.bytes().await?.to_vec()),
        _ => Err("got some other status code".into()),
    }
}

#[allow(dead_code)]
pub async fn download_image_from_image_result(
    image_result: &crate::models::ImageResult,
) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = image_result.base_url.clone();
    let hash = image_result.chapter.hash.clone();

    let download_futures = image_result.chapter.data_saver.iter().map(|image_name| {
        let url = format!("{base_url}/data-saver/{hash}/{image_name}");
        let image_name_clone = image_name.clone();

        tokio::spawn(async move {
            dbg!(&url);
            download_and_save_image(url, image_name_clone).await
        })
    });

    let results = futures::future::join_all(download_futures).await;
    let mut image_names_vec: Vec<String> = Vec::new();
    for result in results {
        match result {
            Ok(Ok(image_name)) => image_names_vec.push(image_name),
            Ok(Err(e)) => eprintln!("Failed to download or save image: {e}"),
            Err(e) => eprintln!("Task join error: {e}"),
        }
    }

    std::process::Command::new("magick").args(image_names_vec.clone()).arg("output.pdf").output()?;

    let del = image_names_vec.into_iter().map(|image_name| {
        tokio::spawn(async move {
            tokio::fs::remove_file(&image_name).await
        })
    });

    let _res = futures::future::join_all(del).await;

    Ok(())
}

#[allow(dead_code)]
pub async fn download_and_save_image(url: String, image_name: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let bytes = get_image_bytes(&url).await.expect("not able to get image bytes");
    save_image_to_file(&image_name, bytes).await.expect("not able to save image");
    Ok(image_name)
}

#[allow(dead_code)]
pub async fn save_image_to_file(image_name: &str, bytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let file = tokio::fs::File::create(image_name).await?;
    let mut writer = tokio::io::BufWriter::new(file);
    writer.write_all(&bytes).await?;
    writer.flush().await?;
    Ok(())
}

#[allow(dead_code)]
pub async fn manga_search_from_search_query(
    search_query: &str,
) -> Result<crate::models::MangaSearch, Box<dyn std::error::Error>> {
    let url = format!("https://api.mangadex.org/manga?title={search_query}");
    let blob = make_api_request(&url)
        .await
        .expect("make api for getting manga search res");
    let manga_res: crate::models::MangaSearch =
        serde_json::from_str(&blob).expect("not able to convert to manga_search");
    return Ok(manga_res);
}

#[allow(dead_code)]
pub async fn chapters_from_manga_id(
    manga_id: &str,
) -> Result<crate::models::ChapterInfo, Box<dyn std::error::Error>> {
    let url = format!("https://api.mangadex.org/manga/{}/feed", manga_id);
    let blob = make_api_request(&url)
        .await
        .expect("get chapters from manga id failed");
    let chapter: crate::models::ChapterInfo =
        serde_json::from_str(&blob).expect("not able to convert to Chapter Info type");
    return Ok(chapter);
}

#[allow(dead_code)]
pub async fn image_res_from_chapter_id(
    chapter_id: &str,
) -> Result<crate::models::ImageResult, Box<dyn std::error::Error>> {
    let url = format!("https://api.mangadex.org/at-home/server/{}", chapter_id);
    let blob = make_api_request(&url)
        .await
        .expect("no res from at-home server shit");
    let image_info: crate::models::ImageResult =
        serde_json::from_str(&blob).expect("not able to convert to image_res type");
    return Ok(image_info);
}
