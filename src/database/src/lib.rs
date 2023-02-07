mod generators;
mod loaders;

pub use loaders::{
    ClubEntity, ClubLoader, ContinentEntity, ContinentLoader, CountryEntity, CountryLoader,
    LeagueEntity, LeagueLoader, NamesByCountryEntity, NamesByCountryLoader,
};

pub use generators::DatabaseGenerator;

pub struct DatabaseEntity {
    pub continents: Vec<ContinentEntity>,
    pub countries: Vec<CountryEntity>,
    pub leagues: Vec<LeagueEntity>,
    pub clubs: Vec<ClubEntity>,

    pub names_by_country: Vec<NamesByCountryEntity>,
}

pub struct DatabaseLoader;

impl DatabaseLoader {
    pub fn load() -> DatabaseEntity {
        DatabaseEntity {
            continents: ContinentLoader::load(),
            countries: CountryLoader::load(),
            leagues: LeagueLoader::load(),
            clubs: ClubLoader::load(),
            names_by_country: NamesByCountryLoader::load(),
        }
    }
}
