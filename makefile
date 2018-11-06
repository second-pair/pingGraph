make:
	pyinstaller.exe --onefile --specpath build --distpath bin pingGraph.py
	cp settings.json bin/settings.json

clean:
	rm -rf bin build __pycache__
