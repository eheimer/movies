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
    pub id: i32,
    pub name: String,
}

#[derive(Clone)]
pub struct Season {
    pub id: i32,
    pub series: Series,
    pub number: i32,
}