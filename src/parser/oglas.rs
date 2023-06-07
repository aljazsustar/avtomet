#[derive(Debug, Clone)]
pub struct Oglas {
    pub naziv: String,
    pub znamka: String,
    pub model: String,
    pub cena: f32,
    pub kilometri: u32,
    pub prva_registracija: u32,
    pub naslov: String
}

impl Default for Oglas {
    fn default() -> Self {
        Self {
            naziv: Default::default(),
            znamka: Default::default(),
            model: Default::default(),
            cena: Default::default(),
            kilometri: Default::default(),
            prva_registracija: Default::default(),
            naslov: Default::default()
        }
    }
}

impl Oglas {
    pub fn new() -> Self {
        Self::default()
    }
}

pub struct OglasBuilder {
    oglas: Oglas,
}

impl OglasBuilder {
    pub fn new() -> Self {
        Self {
            oglas: Oglas::new(),
        }
    }

    pub fn add_naziv(&mut self, naziv: String) -> &mut Self {
        self.oglas.naziv = naziv;
        self
    }

    pub fn add_znamka(&mut self, znamka: String) -> &mut Self {
        self.oglas.znamka = znamka;
        self
    }

    pub fn add_model(&mut self, model: String) -> &mut Self {
        self.oglas.model = model;
         self
    }

    pub fn add_cena(&mut self, cena: f32) -> &mut Self {
        self.oglas.cena = cena;
        self
    }

    pub fn add_kilometri(&mut self, kilometri: u32) -> &mut Self {
        self.oglas.kilometri = kilometri;
        self
    }

    pub fn add_prva_registracija(&mut self, prva_registracija: u32) -> &mut Self {
        self.oglas.prva_registracija = prva_registracija;
        self
    }

    pub fn add_naslov(&mut self, url: String) -> &mut Self {
        self.oglas.naslov = url;
        self
    }

    pub fn build(&mut self) -> Oglas {
        self.oglas.clone()
    }
}
