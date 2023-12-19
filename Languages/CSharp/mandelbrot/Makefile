NTIMES=1

export NTIMES

compile:
	~/.dotnet/dotnet new console --force
	sed -i '/<\/PropertyGroup>/i \    <AllowUnsafeBlocks>true<\/AllowUnsafeBlocks>' obj/mandelbrot.csproj.nuget.g.props
	cp mandelbrot.csharpcore Program.cs
	~/.dotnet/dotnet build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "~/.dotnet/dotnet ./bin/Release/net7.0/mandelbrot.dll.dll 16000" CSharp mandelbrot $(NTIMES)

run:
	~/.dotnet/dotnet ./bin/Release/net7.0/mandelbrot.dll 16000

mem:
	/usr/bin/time -v ~/.dotnet/dotnet ./bin/Release/net7.0/mandelbrot.dll 16000

valgrind:
	valgrind --tool=massif --stacks=yes ~/.dotnet/dotnet ./bin/Release/net7.0/mandelbrot.dll 16000

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj