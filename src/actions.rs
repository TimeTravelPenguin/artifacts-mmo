use crate::{
    api::{self, Action, ArtifactsError, ParamatarisedAction, client::ArtifactsClient},
    models,
};

pub struct GetMonstersAction;

impl ParamatarisedAction for GetMonstersAction {
    type Return = Vec<models::monster::Monster>;
    type Error = api::monsters::GetAllMonstersError;
    type Param = api::monsters::MonsterQuery;

    async fn execute(
        &self,
        api: &ArtifactsClient,
        query: &Self::Param,
    ) -> Result<Self::Return, ArtifactsError<Self::Error>> {
        let monsters: Vec<_> = api.get_monsters(query).await?;
        Ok(monsters)
    }
}

pub struct GetMapsAction;

impl ParamatarisedAction for GetMapsAction {
    type Return = Vec<models::map::Map>;
    type Error = api::maps::GetAllMapsError;
    type Param = api::maps::MapQuery;

    async fn execute(
        &self,
        api: &ArtifactsClient,
        query: &Self::Param,
    ) -> Result<Self::Return, ArtifactsError<Self::Error>> {
        let maps: Vec<_> = api.get_maps(query).await?;
        Ok(maps)
    }
}

pub struct GetCharactersAction;

impl Action for GetCharactersAction {
    type Return = Vec<models::character::Character>;
    type Error = api::my_characters::GetCharactersError;

    async fn execute(
        &self,
        api: &ArtifactsClient,
    ) -> Result<Self::Return, ArtifactsError<Self::Error>> {
        let characters: Vec<_> = api.get_characters().await?;
        Ok(characters)
    }
}

pub struct RestAction;

impl ParamatarisedAction for RestAction {
    type Return = models::rest::CharacterRestData;
    type Error = api::my_characters::CharacterRestError;
    type Param = models::character::Character;

    async fn execute(
        &self,
        api: &ArtifactsClient,
        character: &Self::Param,
    ) -> Result<Self::Return, ArtifactsError<Self::Error>> {
        let character = api.rest(&character.name).await?;
        Ok(character)
    }
}

pub struct FightAction;

impl ParamatarisedAction for FightAction {
    type Return = models::fight::CharacterFightData;
    type Error = api::my_characters::CharacterFightError;
    type Param = models::character::Character;

    async fn execute(
        &self,
        api: &ArtifactsClient,
        character: &Self::Param,
    ) -> Result<Self::Return, ArtifactsError<Self::Error>> {
        let character = api.fight(&character.name).await?;
        Ok(character)
    }
}
