NTIMES=1

export NTIMES

compile:
	~/.dotnet/dotnet new console --force
	sed -i '/<\/PropertyGroup>/i \    <AllowUnsafeBlocks>true<\/AllowUnsafeBlocks>' obj/spectral-norm.csproj.nuget.g.props 
	cp spectralnorm.csharpcore-5.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "~/.dotnet/dotnet ./bin/Release/net7.0/spectral-norm.dll 5500" CSharp spectral-norm $(NTIMES)

run:
	~/.dotnet/dotnet ./bin/Release/net7.0/spectral-norm.dll 5500

mem:
	/usr/bin/time -v  ~/.dotnet/dotnet ./bin/Release/net7.0/spectral-norm.dll 5500

valgrind:
	valgrind --tool=massif --stacks=yes  ~/.dotnet/dotnet ./bin/Release/net7.0/spectral-norm.dll 5500

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj