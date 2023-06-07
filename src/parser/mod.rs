use self::oglas::{Oglas, OglasBuilder};
use scraper::{element_ref::Select, ElementRef, Html, Selector};
use std::fs;

pub mod oglas;

pub fn parse_ads(html: &String) -> Vec<Oglas> {
    let mut oglasi: Vec<Oglas> = Vec::new();
    let document = Html::parse_document(html);

    fs::write("html.html", html).expect("Error writing to file");

    let form = document
        .select(&Selector::parse(r#"form[name="results"]"#).expect("Unable to parse selector"))
        .nth(1)
        .expect("No element for selector was found");

    for el in form
        .select(&Selector::parse("div.GO-Results-Row").expect("No element for selector was found"))
    {

        let mut builder = OglasBuilder::new();
        parse_ad_url(&mut builder, el);
        parse_znamka_model_naziv(&mut builder, el);
        let ad = el
            .select(
                &Selector::parse("div.GO-Results-Data-Top")
                    .expect("Could not parse data div selector"),
            )
            .next()
            .expect("Could not find data div");

        let data_rows = ad
            .select(&Selector::parse("table > tbody").unwrap())
            .next()
            .unwrap();

        let tr_selector = Selector::parse("tbody > tr").unwrap();
        let trs = &mut data_rows.select(&tr_selector);
        get_oglas_from_table_data(&mut builder, trs);
        oglasi.push(builder.build());
    }

    oglasi
}

fn get_oglas_from_table_data<'a>(builder: &'a mut OglasBuilder, elements: &mut Select) -> () {
    let td_selector = Selector::parse("td").unwrap();
    let mut tds_reg = elements.nth(0).unwrap().select(&td_selector);

    let registracija = tds_reg.next().unwrap().text().next().unwrap().to_string();
    let registracija_value = tds_reg.next().unwrap().text().next().unwrap().to_string();

    let mut tds_km = elements.nth(0).unwrap().select(&td_selector);
    let km = tds_km.next().unwrap().text().next().unwrap().to_string();
    let km_value = tds_km.next().unwrap().text().next().unwrap().to_string();

    if registracija == "1.registracija" {
        builder.add_prva_registracija(registracija_value.parse::<u32>().unwrap());
    }

    if km.starts_with("Prevo") {
        builder.add_kilometri(
            km_value.split(" ").collect::<Vec<&str>>()[0]
                .trim()
                .parse::<u32>()
                .unwrap(),
        );
    }
}

fn parse_ad_url<'a>(builder: &'a mut OglasBuilder, div: ElementRef) -> () {
    let url = div
        .select(&Selector::parse("div > a").unwrap())
        .next()
        .unwrap()
        .value()
        .attr("href")
        .unwrap()
        .strip_prefix("../")
        .unwrap();
    builder.add_naslov(format!("https://www.avto.net/{}", url.to_string()));
}

fn parse_znamka_model_naziv<'a>(builder: &'a mut OglasBuilder, div: ElementRef) -> () {
    let text = div.select(&Selector::parse("div > span").unwrap()).next().unwrap().text().next().unwrap();
    let split = text.split(" ").collect::<Vec<&str>>();
    builder.add_naziv(text.to_string());
    builder.add_znamka(split[0].to_string());
    builder.add_model(split[1].to_string());
}