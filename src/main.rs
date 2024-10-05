mod apis;
mod headers;
mod models;
mod table;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = clap::Command::new("manga")
        .version("0.0.1")
        .author("shashank")
        .about("for downloading mangas")
        .subcommand(
            clap::Command::new("search")
                .about("fuzzy search for manga names")
                .arg(
                    clap::Arg::new("search")
                        .help("the search query")
                        .required(true),
                ),
        )
        .subcommand(
            clap::Command::new("chapters").about("give manga hid").arg(
                clap::Arg::new("chapters")
                    .help("manga name to get info about")
                    .required(true),
            ),
        )
        .subcommand(
            clap::Command::new("download")
                .about("give chapter hid")
                .arg(
                    clap::Arg::new("download")
                        .help("the download query")
                        .required(true),
                ),
        )
        .subcommand(
            clap::Command::new("download_all")
                .about("give chapter hid")
                .arg(
                    clap::Arg::new("download_all")
                        .help("the download_all query to download all chapters of a manga")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("search", sub_matches)) => {
            let query = sub_matches.get_one::<String>("search").expect("Required");
            println!("Searching for: {}", query);
            let search_res = apis::manga_search_from_search_query(&query).await?;
            table::display_mangas(&search_res).await?;
        }

        Some(("chapters", sub_matches)) => {
            let chapters = sub_matches.get_one::<String>("chapters").expect("Required");
            let chapter_info_from_id = apis::chapters_from_manga_id(&chapters).await?;
            table::display_chapters(chapter_info_from_id.clone()).await?;
        }

        Some(("download", sub_matches)) => {
            let download = sub_matches.get_one::<String>("download").expect("Required");
            let image_res = apis::image_res_from_chapter_id(&download).await?;
            let mut chapter_name = String::new();
            print!("save with name: ");
            std::io::stdout().flush()?;
            std::io::stdin().read_line(&mut chapter_name)?;
            let chapter_name = chapter_name.trim();
            apis::download_image_from_image_result(&image_res, &chapter_name).await?;
        }
        Some(("download_all", sub_matches)) => {
            let download_all = sub_matches
                .get_one::<String>("download_all")
                .expect("Required");
            let chapter_info = apis::chapters_from_manga_id(&download_all).await?;
            for chapter in chapter_info.data.into_iter().rev() {
                let image_res = apis::image_res_from_chapter_id(&chapter.id).await?;
                apis::download_image_from_image_result(&image_res, &chapter.attributes.title.unwrap_or(chapter.id)).await?;
            }

        }
        _ => println!("Please use a valid subcommand. Use --help for more information."),
    }

    return Ok(());
}
