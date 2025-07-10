use tracing::debug;

use crate::{
    api::{ArtifactsApiResponse, ArtifactsError, client::ArtifactsClient},
    make_error,
    models::{
        character::Character, fight::CharacterFightData, movement::CharacterMovementData,
        rest::CharacterRestData,
    },
};

make_error!(GetCharactersError);

make_error!(CharacterFightError,
    486 => ActionAlreadyInProgressForCharacter
        => "Action already in progress for character",
    497 => CharacterInventoryFull
        => "Character inventory is full",
    498 => CharacterNotFound
        => "Character not found",
    499 => CharacterInCooldown
        => "Character is in cooldown",
    598 => MonsterNotFound
        => "Monster not found"
);

make_error!(CharacterRestError,
    486 => ActionAlreadyInProgressForCharacter
        => "Action already in progress for character",
    498 => CharacterNotFound
        => "Character not found",
    499 => CharacterInCooldown
        => "Character is in cooldown",
);

make_error!(CharacterMoveError,
    404 => MapNotFound
        => "Map not found",
    486 => ActionAlreadyInProgressForCharacter
        => "Action already in progress for character",
    490 => CharacterAlreadyAtLocation
        => "Character already at location",
    498 => CharacterNotFound
        => "Character not found",
    499 => CharacterInCooldown
        => "Character is in cooldown",
);

impl ArtifactsClient {
    /// Fetches all characters for the authenticated user.
    pub async fn get_characters(
        &self,
    ) -> Result<Vec<Character>, ArtifactsError<GetCharactersError>> {
        debug!("Fetching all characters");

        let url = format!("{}/my/characters", self.base_url);
        let resp = self
            .client
            .get(url)
            .bearer_auth(self.api_token.clone())
            .send()
            .await?;

        let chars = ArtifactsApiResponse::<_>::parse_json(resp).await?;
        Ok(chars)
    }

    /// Initiates a fight with the specified character.
    pub async fn fight(
        &self,
        name: &str,
    ) -> Result<CharacterFightData, ArtifactsError<CharacterFightError>> {
        debug!("Fighting with character: {}", name);

        let body = serde_json::json!({
            "name": name,
        });

        let url = format!("{}/my/{}/action/fight", self.base_url, name);
        let resp = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(self.api_token.clone())
            .send()
            .await?;

        let char = ArtifactsApiResponse::<_>::parse_json(resp).await?;
        Ok(char)
    }

    /// Rests the specified character.
    pub async fn rest(
        &self,
        name: &str,
    ) -> Result<CharacterRestData, ArtifactsError<CharacterRestError>> {
        debug!("Resting character: {}", name);

        let body = serde_json::json!({
            "name": name,
        });

        let url = format!("{}/my/{}/action/rest", self.base_url, name);
        let resp = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(self.api_token.clone())
            .send()
            .await?;

        let char = ArtifactsApiResponse::<_>::parse_json(resp).await?;
        Ok(char)
    }

    /// Moves the specified character to the given coordinates.
    pub async fn move_character(
        &self,
        name: &str,
        x: i32,
        y: i32,
    ) -> Result<CharacterMovementData, ArtifactsError<CharacterMoveError>> {
        debug!("Moving character: {} to ({}, {})", name, x, y);

        let body = serde_json::json!({
            "x": x,
            "y": y,
        });

        let url = format!("{}/my/{}/action/move", self.base_url, name);
        let resp = self
            .client
            .post(url)
            .json(&body)
            .bearer_auth(self.api_token.clone())
            .send()
            .await?;

        let char = ArtifactsApiResponse::<_>::parse_json(resp).await?;
        Ok(char)
    }
}
