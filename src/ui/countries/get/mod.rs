﻿use crate::GameAppData;
use actix_web::web::Data;
use actix_web::{web, HttpResponse, Result};
use askama::Template;
use core::Country;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CountryGetRequest {
    country_id: u32,
}

#[derive(Template)]
#[template(path = "countries/get/get.html")]
pub struct CountryGetViewModel<'c> {
    pub id: u32,
    pub name: &'c str,
    pub code: &'c str,
    pub continent_name: &'c str,
    pub leagues: Vec<LeagueDto<'c>>,
}

pub struct LeagueDto<'l> {
    pub id: u32,
    pub name: &'l str,
}

pub async fn country_get_action(
    state: Data<GameAppData>,
    route_params: web::Path<CountryGetRequest>,
) -> Result<HttpResponse> {
    let guard = state.data.lock().await;

    let simulator_data = guard.as_ref().unwrap();

    let country: &Country = simulator_data
        .continents
        .iter()
        .flat_map(|c| &c.countries)
        .find(|country| country.id == route_params.country_id)
        .unwrap();

    let continent = simulator_data.continent(country.continent_id).unwrap();

    let model = CountryGetViewModel {
        id: country.id,
        name: &country.name,
        code: &country.code,
        continent_name: &continent.name,
        leagues: country
            .leagues
            .leagues
            .iter()
            .map(|l| LeagueDto {
                id: l.id,
                name: &l.name,
            })
            .collect(),
    };

    let html = CountryGetViewModel::render(&model).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
