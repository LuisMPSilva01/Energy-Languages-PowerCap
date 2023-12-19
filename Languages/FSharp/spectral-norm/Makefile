NTIMES=1

export NTIMES

compile:
	~/.dotnet/dotnet new console -lang "F#" --force
	cp spectralnorm.fsharpcore-4.fsharpcore Program.fs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/bin/dotnet ./bin/Release/net7.0/spectral-norm.dll 5500" FSharp spectral-norm $(NTIMES)

run:
	/usr/bin/dotnet ./bin/Release/net7.0/spectral-norm.dll 5500

mem:
	/usr/bin/time -v  /usr/bin/dotnet ./bin/Release/net7.0/spectral-norm.dll 5500

valgrind:
	valgrind --tool=massif --stacks=yes  /usr/bin/dotnet ./bin/Release/net7.0/spectral-norm.dll 5500

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.fs
	rm -f *.fsproj