use rand::Rng;

#[derive(Default, Debug)]
struct Options {
    tos: bool,
    tas: bool,
    tng: bool,
    ds9: bool,
    voy: bool,
    ent: bool,
    dis: bool,
    pic: bool,
    ld: bool,
    snw: bool,
    prodigy: bool,

    tmp: bool,
    twok: bool,
    tsfs: bool,
    tvh: bool,
    tff: bool,
    tuc: bool,

    stg: bool,
    stfc: bool,
    sti: bool,
    stn: bool,

    stk: bool,
    stid: bool,
    stb: bool,

    episode: String,
}

impl Options {
    fn num_true(&self) -> i32 {
        let mut sum = 0;

        if self.tos { sum += 1 ;}
        if self.tas { sum += 1 ;}
        if self.tng { sum += 1 ;}
        if self.ds9 { sum += 1 ;}
        if self.voy { sum += 1 ;}
        if self.ent { sum += 1 ;}
        if self.dis { sum += 1 ;}
        if self.pic { sum += 1 ;}
        if self.ld { sum += 1 ;}
        if self.snw { sum += 1 ;}
        if self.prodigy { sum += 1 ;}
        if self.tmp { sum += 1 ;}
        if self.twok { sum += 1 ;}
        if self.tsfs { sum += 1 ;}
        if self.tvh { sum += 1 ;}
        if self.tff { sum += 1 ;}
        if self.tuc { sum += 1 ;}
        if self.stg { sum += 1 ;}
        if self.stfc { sum += 1 ;}
        if self.sti { sum += 1 ;}
        if self.stn { sum += 1 ;}
        if self.stk { sum += 1 ;}
        if self.stid { sum += 1 ;}
        if self.stb { sum += 1 ;}

        sum
    }
}

struct Seasons {
    tos: Vec<i32>,
    tas: Vec<i32>,
    tng: Vec<i32>,
    ds9: Vec<i32>,
    voy: Vec<i32>,
    ent: Vec<i32>,
    dis: Vec<i32>,
    pic: Vec<i32>,
    ld:  Vec<i32>,
    snw: Vec<i32>,
    prodigy: Vec<i32>,
}

impl Default for Seasons {
    fn default() -> Self {
        Self {
            tos: vec![29,26,24],
            tas: vec![16,6],
            tng: vec![26,22,26,26,26,26,26],
            ds9: vec![20,26,26,26,26,26,26],
            voy: vec![16,26,26,26,26,26,26],
            ent: vec![26,26,24,22],
            dis: vec![15,14,13,13],
            pic: vec![10,10,10],
            ld: vec![10,10,10],
            snw: vec![10,10] ,
            prodigy: vec![20],
        }       
    }
}

fn main() {
    let mut options = Options::default();
    options.tng = true;
    let seasons = Seasons::default();
    select_from_options(&mut options, &seasons);
    println!("{}",options.episode);
}

fn select_from_options(options: &mut Options, seasons: &Seasons) {
    if options.num_true() == 0 { 
        options.episode = "Must select options".to_string();
        return;
    }
    let mut num = rand::thread_rng().gen_range(1..=options.num_true());

    if options.tos { num -= 1; if num == 0 { options.episode = format!("The Original Series {}",generate_episode(&seasons.tos)); }}
    if options.tas { num -= 1; if num == 0 { options.episode = format!("The Animated Series {}",generate_episode(&seasons.tas)); }}
    if options.tng { num -= 1; if num == 0 { options.episode = format!("The Next Generation {}",generate_episode(&seasons.tng)); }}
    if options.ds9 { num -= 1; if num == 0 { options.episode = format!("Deep Space 9 {}",generate_episode(&seasons.ds9)); }}
    if options.voy { num -= 1; if num == 0 { options.episode = format!("Voyager {}",generate_episode(&seasons.voy)); }}
    if options.ent { num -= 1; if num == 0 { options.episode = format!("Enterprise {}",generate_episode(&seasons.ent)); }}
    if options.dis { num -= 1; if num == 0 { options.episode = format!("Discovery {}",generate_episode(&seasons.dis)); }}
    if options.pic { num -= 1; if num == 0 { options.episode = format!("Picard {}",generate_episode(&seasons.pic)); }}
    if options.ld { num -= 1; if num == 0 { options.episode = format!("Lower Decks {}",generate_episode(&seasons.ld)); }}
    if options.snw { num -= 1; if num == 0 { options.episode = format!("Strange New Worlds {}",generate_episode(&seasons.snw)); }}
    if options.prodigy { num -= 1; if num == 0 { options.episode = format!("Prodigy {}",generate_episode(&seasons.prodigy)); }}
    if options.tmp { num -= 1; if num == 0 { options.episode = "Movie: The Motion Picture".to_string(); }}
    if options.twok { num -= 1; if num == 0 { options.episode = "Movie: The Wrath of Khan".to_string(); }}
    if options.tsfs { num -= 1; if num == 0 { options.episode = "Movie: The Search for Spock".to_string(); }}
    if options.tvh { num -= 1; if num == 0 { options.episode = "Movie: The Voyage Home".to_string(); }}
    if options.tff { num -= 1; if num == 0 { options.episode = "Movie: The Final Frontier".to_string(); }}
    if options.tuc { num -= 1; if num == 0 { options.episode = "Movie: The Undiscovered Country".to_string(); }}
    if options.stg { num -= 1; if num == 0 { options.episode = "Movie: Star Trek: Generations".to_string(); }}
    if options.stfc { num -= 1; if num == 0 { options.episode = "Movie: Star Trek: First Contact".to_string(); }}
    if options.sti { num -= 1; if num == 0 { options.episode = "Movie: Star Trek: Insurrection".to_string(); }}
    if options.stn { num -= 1; if num == 0 { options.episode = "Movie: Star Trek: Nemesis".to_string(); }}
    if options.stk { num -= 1; if num == 0 { options.episode = "Movie: Star Trek (2009)".to_string(); }}
    if options.stid { num -= 1; if num == 0 { options.episode = "Movie: Star Trek - Into Darkness".to_string(); }}
    if options.stb { num -= 1; if num == 0 { options.episode = "Movie: Star Trek - Beyond".to_string(); }}
}

fn generate_episode(seasons: &Vec<i32>) -> String {
    let season = rand::thread_rng().gen_range(1..=seasons.len());
    let episode = rand::thread_rng().gen_range(1..=seasons[season-1]);
    format!("S{}E{}",season,episode)

}