#[derive(Clone)]
pub struct EpisodeDetail {
    pub title: String,
    pub year: String,
    pub watched: String,
    pub length: String,
    pub series: Option<Series>,
    pub season: Option<Season>,
    pub episode_number: String,
    pub last_watched_time: Option<String>,
    pub last_progress_time: Option<String>,
}

#[derive(Clone)]
pub struct Series {
    pub id: usize,
    pub name: String,
}

#[derive(Clone)]
pub struct Season {
    pub id: usize,
    pub number: usize,
}
