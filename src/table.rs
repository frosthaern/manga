use std::io::Write;

#[allow(dead_code)]
pub async fn display_mangas(
    manga_res: &crate::models::MangaSearch,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut table = prettytable::Table::new();

    for (i, manga) in manga_res.data.iter().enumerate() {
        let title = manga
            .attributes
            .as_ref()
            .unwrap()
            .title
            .as_ref()
            .unwrap()
            .en
            .clone()
            .unwrap();
        // let desc = manga.attributes.as_ref().unwrap().description.as_ref().unwrap().en.clone().unwrap();
        table.add_row(prettytable::row![i.to_string(), manga.id, title]);
    }

    let mut less = std::process::Command::new("less")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("not able to call less");

    less.stdin
        .as_mut()
        .unwrap()
        .write_all(table.to_string().as_bytes())?;
    less.wait().expect("less error");
    return Ok(());
}

#[allow(dead_code)]
pub async fn display_chapters(
    chapter_id: crate::models::ChapterInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut table = prettytable::Table::new();

    table.add_row(prettytable::row!["sno", "chapter_id", "chapter_no", "volume", "title"]);
    for (i, chapter) in chapter_id.data.into_iter().rev().enumerate() {
        let title = chapter.attributes.title.unwrap_or("N/A".to_string());
        let volume = chapter.attributes.volume.unwrap_or("N/A".to_string());
        let chapter_no = chapter.attributes.chapter.unwrap_or("N/A".to_string());
        let chapter_id = chapter.id;
        table.add_row(prettytable::row![i.to_string(), chapter_id, chapter_no, volume, title]);
    }

    let mut less = std::process::Command::new("less")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("not able to call less");

    less.stdin
        .as_mut()
        .unwrap()
        .write_all(table.to_string().as_bytes())?;
    less.wait().expect("less error");
    return Ok(());
}
