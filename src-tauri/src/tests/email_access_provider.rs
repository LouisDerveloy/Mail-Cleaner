/*
 * Copyright (c) 2025. Louis DERVELOY (https://github.com/LouisDerveloy/)
 * Licensed under the PolyForm Noncommercial License 1.0.0
 * SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0
 * See the LICENSE file in the project root for details.
 */
use imap::Authenticator;
use crate::email_access_provider;

#[test]
fn oauth_credentials_process_test() {

    let cred = email_access_provider::OAuthCredentials::new("alice@example.com".into(), "ya29.A0AfH6S...".into());
    let res = cred.process(&[]);

    assert!(
        res.starts_with("user=alice@example.com\x01auth=Bearer ya29."),
        "SASL XOAUTH2 should start with 'user=...\\x01auth=Bearer ...'"
    );
}