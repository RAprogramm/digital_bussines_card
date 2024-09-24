.PHONY: release

release:
	trunk build --release -M && firebase deploy --only hosting:raprogramm
