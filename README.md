# Rausth

Système d'authentification modulaire en Rust, conçu pour fonctionner 
sur plusieurs environnements (web, systèmes classiques, embarqué) 
via un cœur d'identité découplé des protocoles de transport.

## Statut du projet
🚧 En développement actif — Brique 1/3 : cœur + adaptateur web

## Architecture

Le projet sépare :
- **`core/`** — logique d'authentification pure (hashage, vérification, 
  émission d'identité), indépendante du réseau ou du protocole
- **`adapters/`** — implémentations spécifiques à chaque environnement 
  (web aujourd'hui, embarqué/système classique prévus ensuite)

## Roadmap
- [x] Cœur d'identité (Argon2, trait UserRepository)
- [ ] Adaptateur web (axum, JWT, SQLite)
- [ ] Adaptateur système classique
- [ ] Adaptateur embarqué

## Stack technique
- Rust (workspace Cargo)
- axum (serveur HTTP)
- sqlx + SQLite (persistance)
- argon2 (hashage des mots de passe)
- jsonwebtoken (émission de tokens)

## Pourquoi ce projet


## Licence
Apache 2.0
