# Mailmanage

Mailmanage is free, open‑source software that helps you clean your inbox by deleting undesirable advertising emails quickly and safely.

## Getting started

You can install Mailmanage in two ways:

- Recommended: Download the installer from the project releases.
- Build from source: Clone the repository and run the app (requires Node.js and Rust).

### Recommended: use the installer

1. Go to the project’s [Releases](https://github.com/LouisDerveloy/Mail-Cleaner/releases) page.
2. Download the latest installer from the Assets section.
3. Run the installer and follow the on‑screen steps.

### Build from source

Prerequisites:
- Git
- Node.js (LTS recommended)
- Rust (stable toolchain)

Clone the repository:
```sh
git clone https://github.com/LouisDerveloy/Mail-Cleaner.git
cd Mail-Cleaner
```

Run the app in development:
```sh
npm run tauri dev
```

Create a production build/installer:
```sh
npm run tauri build
```

# How to use
1. Launch the app. The main page opens, where you will delete emails.
   First, grant the app access to your inbox: click the “Sign in” button at the top.
2. On the sign‑in page, choose a connection method: 
   - Email and password (if supported by your provider).
   - OAuth 2.0.
   - (recommended if available) Third‑party login. Currently, only Google (Gmail) is supported.
     
   Important: 
   - Your email provider must allow IMAP for this app to work.
   - Some providers only allow OAuth 2.0 authentication.
3. After signing in successfully, you’ll return to the main page.
Start a search using the search bar (top‑left).
You can leave it empty and press the search icon, or open the Advanced Search.
4. When the search finishes, results appear as rows in a table.
Each row represents a sender (one email address).
Companies often use multiple addresses, for example:
   - newsletter@company.com
   - security@company.com
   - no-reply@company.com

   Select only the addresses that send you undesirable emails. You can select a few to try first.
5. Double‑check your selection. Then click “Delete selected”.
   **Warning**: All emails you received from the selected addresses will be deleted.
   Depending on your provider, you may be able to recover them from Trash.

## Privacy & Data

- No data collection, no telemetry.
- All processing happens locally on your machine.
- Network connections are only made to your email provider (IMAP/OAuth) to perform the actions you request.
- Credentials are not saved so you will have to authenticate each time you close the app.

## Architecture (local‑first, no server)

- Frontend: Vue 3 + TypeScript
- Desktop wrapper: Tauri
- Local backend: Rust (inside the Tauri process)
- Storage: local app files + OS keyring/secure storage
- Network: IMAP/OAuth to your email provider only — no additional services

## Report a bug

- If you find a bug please open an [issues here](https://github.com/LouisDerveloy/Mail-Cleaner/issues).
- If you find a **security** issues report it [here](https://github.com/LouisDerveloy/Mail-Cleaner/security) and do not open a public issues.