NTIMES=20

export NTIMES

compile:
	~/.dotnet/dotnet new console -lang "F#" --force
	cp binarytrees.fsharpcore-7.fsharpcore Program.fs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained


measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "/usr/bin/dotnet ./bin/Release/net7.0/binary-trees.dll 21" FSharp binary-trees $(NTIMES)

run:
	/usr/bin/dotnet ./bin/Release/net7.0/binary-trees.dll 21

mem:
	/usr/bin/time -v /usr/bin/dotnet ./bin/Release/net7.0/binary-trees.dll 21

valgrind:
	valgrind --tool=massif --stacks=yes /usr/bin/dotnet ./bin/Release/net7.0/binary-trees.dll 21

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.fs
	rm -f *.csproj