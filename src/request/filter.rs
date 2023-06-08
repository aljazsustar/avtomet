use serde::{Serialize, ser::SerializeStruct};

#[derive(Clone)]
pub struct Filter {
    pub znamka: String,
    pub model: String,
    pub model_id: String,
    pub tip: String,
    pub znamka2: String,
    pub model2: String,
    pub tip2: String,
    pub znamka3: String,
    pub model3: String,
    pub tip3: String,
    pub cena_min: String,
    pub cena_max: String,
    pub letnik_min: String,
    pub letnik_max: String,
    pub bencin: String,
    pub starost2: String,
    pub oblika: String,
    pub ccm_min: String,
    pub ccm_max: String,
    pub moc_min: String,
    pub moc_max: String,
    pub km_min: String,
    pub km_max: String,
    pub kw_min: String,
    pub kw_max: String,
    pub motortakt: String,
    pub motorvalji: String,
    pub lokacija: String,
    pub sirina: String,
    pub dolzina: String,
    pub dolzina_min: String,
    pub dolzina_max: String,
    pub nosilnost_min: String,
    pub nosilnost_max: String,
    pub lezisc: String,
    pub presek: String,
    pub premer: String,
    pub col: String,
    pub vijakov: String,
    pub etoznaka: String,
    pub vozilo: String,
    pub airbag: String,
    pub barva: String,
    pub barvaint: String,
    pub eq1: String,
    pub eq2: String,
    pub eq3: String,
    pub eq4: String,
    pub eq5: String,
    pub eq6: String,
    pub eq7: String,
    pub eq8: String,
    pub eq9: String,
    pub kat: String,
    pub pia: String,
    pub pia_zero: String,
    pub pia_out: String,
    pub pslo: String,
    pub akcija: String,
    pub paketgarancije: String,
    pub broker: String,
    pub prikazkategorije: String,
    pub kategorija: String,
    pub onlvid: String,
    pub onlnak: String,
    pub zaloga: String,
    pub arhiv: String,
    pub presort: String,
    pub tipsort: String,
    pub stran: String,
    pub subSORT: String,
    pub subTIPSORT: String
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            znamka: String::from("Audi"),
            model: String::from(""),
            model_id: String::from(""),
            tip: String::from("katerikoli tip"),
            znamka2: String::from(""),
            model2: String::from(""),
            tip2: String::from("katerikoli tip"),
            znamka3: String::from(""),
            model3: String::from(""),
            tip3: String::from("katerikoli tip"),
            cena_min: String::from("0"),
            cena_max:String::from("999999"),
            letnik_min: String::from("0"),
            letnik_max: String::from("2090"),
            bencin: String::from("0"),
            starost2: String::from("999"),
            oblika: String::from("0"),
            ccm_min: String::from("0"),
            ccm_max: String::from("9999"),
            moc_min: String::from(""),
            moc_max: String::from(""),
            km_min: String::from("0"),
            km_max: String::from("9999999"),
            kw_min: String::from("0"),
            kw_max: String::from("999"),
            motortakt: String::from(""),
            motorvalji: String::from(""),
            lokacija: String::from("0"),
            sirina: String::from(""),
            dolzina: String::from("0"),
            dolzina_min: String::from("0"),
            dolzina_max: String::from("100"),
            nosilnost_min: String::from("0"),
            nosilnost_max: String::from("999999"),
            lezisc: String::from(""),
            presek: String::from("0"),
            premer: String::from("0"),
            col: String::from("0"),
            vijakov: String::from("0"),
            etoznaka: String::from("0"),
            vozilo: String::from(""),
            airbag: String::from(""),
            barva: String::from(""),
            barvaint: String::from(""),
            eq1: String::from("1000000000"),
            eq2: String::from("1000000000"),
            eq3: String::from("1000000000"),
            eq4: String::from("1000000000"),
            eq5: String::from("1000000000"),
            eq6: String::from("1000000000"),
            eq7: String::from("1000000120"),
            eq8: String::from("101000000"),
            eq9: String::from("1000000000"),
            kat: String::from("1010000000"),
            pia: String::from(""),
            pia_zero: String::from(""),
            pia_out: String::from(""),
            pslo: String::from(""),
            akcija: String::from("0"),
            paketgarancije: String::from(""),
            broker: String::from("0"),
            prikazkategorije: String::from("0"),
            kategorija: String::from("0"),
            onlvid: String::from("0"),
            onlnak: String::from("0"),
            zaloga: String::from("10"),
            arhiv: String::from("0"),
            presort: String::from("3"),
            tipsort: String::from("DESC"),
            stran: String::from(""),
            subSORT: String::from("2"),
            subTIPSORT: String::from("ASC")
        }   
    }
}

impl Serialize for Filter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut s = serializer.serialize_struct("Filter", 69)?;
        s.serialize_field("znamka", &self.znamka)?;
        s.serialize_field("model", &self.model)?;
        s.serialize_field("modelID", &self.model_id)?;
        s.serialize_field("tip", &self.znamka)?;
        s.serialize_field("znamka2", &self.znamka2)?;
        s.serialize_field("model2", &self.model2)?;
        s.serialize_field("tip2", &self.tip2)?;
        s.serialize_field("znamka3", &self.znamka3)?;
        s.serialize_field("model3", &self.model3)?;
        s.serialize_field("tip3", &self.tip3)?;
        s.serialize_field("cenaMin", &self.cena_min)?;
        s.serialize_field("cenaMax", &self.cena_max)?;
        s.serialize_field("letnikMin", &self.letnik_min)?;
        s.serialize_field("letnikMax", &self.letnik_max)?;
        s.serialize_field("bencin", &self.bencin)?;
        s.serialize_field("starost2", &self.starost2)?;
        s.serialize_field("oblika", &self.oblika)?;
        s.serialize_field("ccmMin", &self.ccm_min)?;
        s.serialize_field("ccmMax", &self.ccm_max)?;
        s.serialize_field("mocMin", &self.moc_min)?;
        s.serialize_field("mocMax", &self.moc_max)?;
        s.serialize_field("kmMin", &self.km_min)?;
        s.serialize_field("kmMax", &self.km_max)?;
        s.serialize_field("kwMin", &self.kw_min)?;
        s.serialize_field("kwMax", &self.kw_max)?;
        s.serialize_field("motortakt", &self.motortakt)?;
        s.serialize_field("motorvalji", &self.motorvalji)?;
        s.serialize_field("lokacija", &self.lokacija)?;
        s.serialize_field("sirina", &self.sirina)?;
        s.serialize_field("dolzina", &self.dolzina)?;
        s.serialize_field("dolzinaMIN", &self.dolzina_min)?;
        s.serialize_field("dolzinaMAX", &self.dolzina_max)?;
        s.serialize_field("nosilnostMIN", &self.nosilnost_min)?;
        s.serialize_field("nosilnostMAX", &self.nosilnost_max)?;
        s.serialize_field("lezisc", &self.lezisc)?;
        s.serialize_field("presek", &self.presek)?;
        s.serialize_field("premer", &self.premer)?;
        s.serialize_field("col", &self.col)?;
        s.serialize_field("vijakov", &self.vijakov)?;
        s.serialize_field("ETozanaka", &self.etoznaka)?;
        s.serialize_field("vozilo", &self.vozilo)?;
        s.serialize_field("airbag", &self.airbag)?;
        s.serialize_field("barva", &self.barva)?;
        s.serialize_field("barvaint", &self.barvaint)?;
        s.serialize_field("EQ1", &self.eq1)?;
        s.serialize_field("EQ2", &self.eq2)?;
        s.serialize_field("EQ3", &self.eq3)?;
        s.serialize_field("EQ4", &self.eq4)?;
        s.serialize_field("EQ5", &self.eq5)?;
        s.serialize_field("EQ6", &self.eq6)?;
        s.serialize_field("EQ7", &self.eq7)?;
        s.serialize_field("EQ8", &self.eq8)?;
        s.serialize_field("EQ9", &self.eq9)?;
        s.serialize_field("KAT", &self.kat)?;
        s.serialize_field("PIA", &self.pia)?;
        s.serialize_field("PIAzero", &self.pia_zero)?;
        s.serialize_field("PIAOut", &self.pia_out)?;
        s.serialize_field("PSLO", &self.pslo)?;
        s.serialize_field("akcija", &self.akcija)?;
        s.serialize_field("paketgarancije", &self.paketgarancije)?;
        s.serialize_field("broker", &self.broker)?;
        s.serialize_field("prikazkatergorije", &self.prikazkategorije)?;
        s.serialize_field("kategorija", &self.kategorija)?;
        s.serialize_field("ONLvid", &self.onlvid)?;
        s.serialize_field("ONLnak", &self.onlnak)?;
        s.serialize_field("zaloga", &self.zaloga)?;
        s.serialize_field("arhiv", &self.arhiv)?;
        s.serialize_field("presort", &self.presort)?;
        s.serialize_field("tipsort", &self.tipsort)?;
        s.serialize_field("stran", &self.stran)?;
        s.serialize_field("subSORT", &self.subSORT)?;
        s.serialize_field("subTIPSORT", &self.subTIPSORT)?;
        s.end()
    }
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }
}


pub struct FilterBuilder {
    filter: Filter,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            filter: Filter::default(),
        }
    }

    pub fn add_znamka(&mut self, znamka: String) -> &mut Self {
        self.filter.znamka = znamka;
        self
    }

    pub fn add_model(&mut self, model: String) -> &mut Self {
        self.filter.model = model;
        self
    }

    pub fn add_km_min(&mut self, km_min: u16) -> &mut Self {
        self.filter.km_min = km_min.to_string();
        self
    }

    pub fn add_km_max(&mut self, km_max: u16) -> &mut Self {
        self.filter.km_max = km_max.to_string();
        self
    }

    pub fn build(&mut self) -> Filter {
        self.filter.clone()
    }
}
