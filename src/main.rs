use dotenv::dotenv;
use std::env;
use unsplash;
use wallpaper::change_wallpaper;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let path = env::current_dir().unwrap().join("photo.jpg");
    unsplash::download_photo(&path).await;
    change_wallpaper(path);
}
