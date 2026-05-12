# kei-contacts-google

Thin client for the [Google People API v1](https://developers.google.com/people).

Consumes a pre-acquired OAuth2 access token (e.g. from `kei-auth-google`).
Does **not** perform OAuth itself.

## Usage

```rust
use kei_contacts_google::GooglePeopleClient;
use kei_social_store::{Store, people::add_person};

async fn sync_contacts(token: String, store: &Store) -> anyhow::Result<()> {
    let client = GooglePeopleClient::new(token);
    let contacts = client.list_connections().await?;
    for c in &contacts {
        let person = c.to_person();
        add_person(store, &person)?;
    }
    Ok(())
}
```

## Output shape

Each `GoogleContact` maps to `kei_social_store::Person`:

| Person field   | Source                           |
|----------------|----------------------------------|
| `name`         | `display_name` or `given family` |
| `email`        | first email address              |
| `organization` | first organization name          |
| `source`       | `"google:{resource_name}"`       |
| `bio`          | first biography value            |
