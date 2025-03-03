#[derive(Clone)]
pub struct EpisodeDetail{
    pub title: String,
    pub year: String,
    pub watched: String,
    pub length: String,
    pub series: Option<Series>,
    pub season: Option<Season>,
    pub episode_number: String,
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