# Business Digital Card

[![Build and Deploy](https://github.com/RAprogramm/digital_bussines_card/actions/workflows/deploy.yml/badge.svg?branch=main)](https://github.com/RAprogramm/digital_bussines_card/actions/workflows/deploy.yml)

## Continuous deployment

The GitHub Actions workflow deploys the site to Firebase Hosting after a successful build. Set the `FIREBASE_SERVICE_ACCOUNT` repository secret to a JSON service account key with Firebase Hosting admin permissions so the workflow can authenticate without deprecated tokens.
