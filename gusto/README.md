A fully generated, opinionated API client library for Gusto.

This library is generated from the Gusto OpenAPI
specs. This way it will remain
up to date as features are added. The documentation for the crate is generated
along with the code to make this library easy to use.

To install the library, add the following to your `Cargo.toml` file.

```toml
[dependencies]
gusto_api = "0.2.0"
```

## Basic example

Typical use will require intializing a `Client`. This requires
a user agent string and set of credentials.

```
use gusto_api::Client;

let gusto = Client::new(
    String::from("client-id")
    String::from("client-secret")
    String::from("redirect-uri")
    String::from("token")
    String::from("refresh-token")
    String::from("company-id")
);
```

Alternatively, the library can search for most of the variables required for
the client in the environment:

- `GUSTO_CLIENT_ID`
- `GUSTO_CLIENT_SECRET`
- `GUSTO_REDIRECT_URI`

And then you can create a client from the environment.

```
use gusto_api::Client;

let gusto = Client::new_from_env(
    String::from("token")
    String::from("refresh-token")
    String::from("company-id")
);
```

It is okay to pass empty values for token, refresh_token, and company_id. In
the initial state of the client, you will not know these values.

To start off a fresh client and get a token and refresh_token, use the following.

```
use gusto_api::Client;

let gusto = Client::new_from_env("", "", "");

// Get the URL to request consent from the user.
let user_consent_url = gusto.user_consent_url();

// In your redirect URL capture the code sent.
// Send it along to the request for the token.
let code = "thing-from-redirect-url";
let mut access_token = gusto.get_access_token(code).unwrap();

// You can additionally refresh the access token with the following.
// You must have a refresh token to be able to call this function.
access_token = gusto.refresh_access_token().unwrap();
```