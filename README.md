[![Continuous Deployment](https://github.com/digitalpalitools/enhanced-digital-pali-dictionary/workflows/Continuous%20Deployment/badge.svg)](https://github.com/digitalpalitools/lib/actions?query=workflow%3A%22Continuous+Deployment%22) [![License: CC BY-NC-SA 4.0](https://img.shields.io/badge/License-CC%20BY--NC--SA%204.0-lightgrey.svg)](https://creativecommons.org/licenses/by-nc-sa/4.0/)

# Enhanced Digital Pāli Dictionary

## Purpose

All infrastructure & work related to dictionaries. Medium term this delivers misc dictionaries such as VRI, DPD. Long term: Enhanced Digital Pāli-LanguageX dictionary.

## Features

## Commands

- Build: ```cargo clean; cargo build```
- Test: ```cargo test```
- Format: ```cargo clean; cargo fmt --all -- --check```
- Clippy: ```cargo clean; cargo clippy --tests --all-targets --all-features -- -D warnings```
- Watch Tests: ```cargo watch -x test```
