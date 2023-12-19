NTIMES=1

export NTIMES

compile:
	~/.dotnet/dotnet new console --force
	sed -i '/<\/PropertyGroup>/i \    <AllowUnsafeBlocks>true<\/AllowUnsafeBlocks>' obj/regex-redux.csproj.nuget.g.props
	cp regexredux.csharpcore-8.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "~/.dotnet/dotnet ./bin/Release/net7.0/regex-redux.dll 0 < ../../../inputs/regexredux-input5000000.txt" CSharp regex-redux $(NTIMES)

run:
	~/.dotnet/dotnet ./bin/Release/net7.0/regex-redux.dll 0 < ../../../inputs/regexredux-input5000000.txt

mem:
	/usr/bin/time -v ~/.dotnet/dotnet ./bin/Release/net7.0/regex-redux.dll 0 < ../../../inputs/regexredux-input5000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes ~/.dotnet/dotnet ./bin/Release/net7.0/regex-redux.dll 0 < ../../../inputs/regexredux-input5000000.txt

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj