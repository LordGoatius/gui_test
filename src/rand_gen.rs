use rand::Rng;

#[derive(Default, Debug)]
pub struct Options {
    pub tos: bool,
    pub tas: bool,
    pub tng: bool,
    pub ds9: bool,
    pub voy: bool,
    pub ent: bool,
    pub dis: bool,
    pub pic: bool,
    pub ld: bool,
    pub snw: bool,
    pub prodigy: bool,

    pub tmp: bool,
    pub twok: bool,
    pub tsfs: bool,
    pub tvh: bool,
    pub tff: bool,
    pub tuc: bool,

    pub stg: bool,
    pub stfc: bool,
    pub sti: bool,
    pub stn: bool,

    pub stk: bool,
    pub stid: bool,
    pub stb: bool,

    pub show_movies: bool,

    pub episode: String,
}

impl Options {
    pub fn num_true(&self) -> i32 {
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

    pub fn all_shows(&mut self) {
        self.tos = true;
        self.tas = true;
        self.tng = true;
        self.ds9 = true;
        self.voy = true;
        self.ent = true;
        self.dis = true;
        self.pic = true;
        self.ld = true;
        self.snw = true;
        self.prodigy = true;
    }

    pub fn all_movies(&mut self) {
        self.tmp = true;
        self.twok = true;
        self.tsfs = true;
        self.tvh = true;
        self.tff = true;
        self.tuc = true;

        self.stg = true;
        self.stfc = true;
        self.sti = true;
        self.stn = true;

        self.stk = true;
        self.stid = true;
        self.stb = true;
        self.show_movies = true;
    }

    pub fn big_3_shows(&mut self) {
        self.tng = true;
        self.ds9 = true;
        self.voy = true;
    }

    pub fn shows_on_ship(&mut self) {
        self.tng = true;
        self.ds9 = true;
        self.tos = true;
        self.ent = true;
        self.snw = true;
        self.dis = true;
        self.ld = true;
    }

    pub fn classic(&mut self) {
        self.tos = true;
        self.tas = true;
        self.tng = true;
        self.ds9 = true;
        self.voy = true;
        self.ent = true;
    }

    pub fn new(&mut self) {
        self.dis = true;
        self.pic = true;
        self.ld = true;
        self.snw = true;
        self.prodigy = true;
    }

    pub fn classic_movies(&mut self) {
        self.tmp = true;
        self.twok = true;
        self.tsfs = true;
        self.tvh = true;
        self.tff = true;
        self.tuc = true;
        self.show_movies = true;
    }

    pub fn tng_movies(&mut self) {
        self.stg = true;
        self.stfc = true;
        self.sti = true;
        self.stn = true;
        self.show_movies = true;
    }
    
    pub fn kelvin_movies(&mut self) {
        self.stk = true;
        self.stid = true;
        self.stb = true;
        self.show_movies = true;
    }
    
    pub fn a_fun_time(&mut self) {
        self.tos = true;
        self.tng = true;
        self.ent = true;
        self.ld = true;
        self.snw = true;
    }

    pub fn jimmys_recommended(&mut self) {
        self.tos = true;
        self.tng = true;
        self.ds9 = true;
        self.voy = true;
        self.ent = true;
        self.pic = true;
        self.ld = true;
        self.snw = true;
        self.tmp = true;
        self.twok = true;
        self.tvh = true;
        self.tuc = true;
        self.stg = true;
        self.stfc = true;    
        self.stk = true;
        self.stid = true;
        self.stb = true;
        self.show_movies = true;
    }

    pub fn deselect_all(&mut self) {
        self.tos = false;
        self.tas = false;
        self.tng = false;
        self.ds9 = false;
        self.voy = false;
        self.ent = false;
        self.dis = false;
        self.pic = false;
        self.ld = false;
        self.snw = false;
        self.prodigy = false;
    
        self.tmp = false;
        self.twok = false;
        self.tsfs = false;
        self.tvh = false;
        self.tff = false;
        self.tuc = false;
    
        self.stg = false;
        self.stfc = false;
        self.sti = false;
        self.stn = false;
    
        self.stk = false;
        self.stid = false;
        self.stb = false;
    }
}

pub struct Seasons {
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

// fn main() {
//     let seasons = Seasons::default();
//     select_from_options(&mut options, &seasons);
//     println!("{}",options.episode);
// }

pub fn select_from_options(options: &mut Options, seasons: &Seasons) {
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
    if options.tmp { num -= 1; if num == 0 { options.episode = "The Motion Picture".to_string(); }}
    if options.twok { num -= 1; if num == 0 { options.episode = "The Wrath of Khan".to_string(); }}
    if options.tsfs { num -= 1; if num == 0 { options.episode = "The Search for Spock".to_string(); }}
    if options.tvh { num -= 1; if num == 0 { options.episode = "The Voyage Home".to_string(); }}
    if options.tff { num -= 1; if num == 0 { options.episode = "The Final Frontier".to_string(); }}
    if options.tuc { num -= 1; if num == 0 { options.episode = "The Undiscovered Country".to_string(); }}
    if options.stg { num -= 1; if num == 0 { options.episode = "Star Trek: Generations".to_string(); }}
    if options.stfc { num -= 1; if num == 0 { options.episode = "Star Trek: First Contact".to_string(); }}
    if options.sti { num -= 1; if num == 0 { options.episode = "Star Trek: Insurrection".to_string(); }}
    if options.stn { num -= 1; if num == 0 { options.episode = "Star Trek: Nemesis".to_string(); }}
    if options.stk { num -= 1; if num == 0 { options.episode = "Star Trek (2009)".to_string(); }}
    if options.stid { num -= 1; if num == 0 { options.episode = "Star Trek - Into Darkness".to_string(); }}
    if options.stb { num -= 1; if num == 0 { options.episode = "Star Trek - Beyond".to_string(); }}
}

fn generate_episode(seasons: &Vec<i32>) -> String {
    let season = rand::thread_rng().gen_range(1..=seasons.len());
    let episode = rand::thread_rng().gen_range(1..=seasons[season-1]);
    format!("S{}E{}",season,episode)
}