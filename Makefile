.PHONY: help db dev prod build-prod down logs ps size nuke

## help       : liste les commandes
help:
	@grep -E '^## ' $(MAKEFILE_LIST) | sed 's/^## //'

## db         : lance uniquement Postgres (pour cargo leptos watch hors Docker)
db:
	docker compose up -d db

## dev        : lance db + serveur de dev (hot reload)
dev:
	docker compose --profile dev up --build

## prod       : build l'image prod puis lance db + app
prod:
	docker compose --profile prod up --build

## build-prod : build l'image prod sans la lancer
build-prod:
	docker compose --profile prod build

## down       : arrête tous les conteneurs (tous profils), garde les données
down:
	docker compose --profile "*" down

## logs       : suit les logs de tous les services
logs:
	docker compose --profile "*" logs -f

## ps         : liste les conteneurs du projet
ps:
	docker compose --profile "*" ps

## size       : taille des images du projet
size:
	docker images --filter "reference=pioche-*"

## nuke       : arrête tout ET supprime les volumes (base de données incluse !)
nuke:
	@read -p "Supprimer tous les volumes, dont la base de données ? [y/N] " ok && [ "$$ok" = y ]
	docker compose --profile "*" down -v
