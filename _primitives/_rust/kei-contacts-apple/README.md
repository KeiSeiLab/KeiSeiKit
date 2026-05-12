# kei-contacts-apple

CardDAV client for iCloud Contacts.

## Authentication

iCloud Contacts uses **app-specific passwords**, not OAuth / Sign in with Apple
(that scope does not include contacts access).

1. Go to <https://appleid.apple.com> → Sign-in and Security → App-Specific Passwords.
2. Click **Generate an app-specific password**.
3. Copy the generated password (format: `xxxx-xxxx-xxxx-xxxx`).

Use your Apple ID e-mail and this password with the client.

## Addressbook URL

Full CardDAV discovery is not yet implemented. Look up your addressbook URL from
a CalDAV/CardDAV client (e.g. Thunderbird → Settings → Address Books → iCloud
account → right-click → Properties) or use the standard iCloud pattern:

```
https://pXX-contacts.icloud.com/<DSID>/carddavhome/card/
```

Where `pXX` is your regional shard and `DSID` is your iCloud account ID.

## Usage

```rust
let client = kei_contacts_apple::ICloudCardDavClient::new(
    "user@icloud.com".to_string(),
    std::env::var("APPLE_APP_SPECIFIC_PASSWORD").unwrap(),
)
.with_addressbook_url(
    "https://p01-contacts.icloud.com/123456789/carddavhome/card/".to_string(),
);

let contacts = client.list_contacts().await?;
for c in &contacts {
    let person = c.to_person(); // -> kei_social_store::people::Person
    println!("{} <{}>", person.name, person.email);
}
```

## Limitations

- Line-folding in vCards (continuation lines starting with a space) is not handled.
- CardDAV discovery (`PROPFIND .well-known/carddav`) is not implemented; supply
  the addressbook URL directly via `with_addressbook_url`.
- Pagination is not implemented; all contacts in the addressbook are returned in
  a single REPORT request.
