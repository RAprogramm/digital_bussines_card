dev:
	yarn serve

build:
	yarn build

yarn prod:
	yarn build
	firebase deploy --only hosting:raprogramm
