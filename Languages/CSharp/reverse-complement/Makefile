NTIMES=20

export NTIMES

include ../../../config.env

ifndef DOTNET_PATH
$(error DOTNET_PATH is not set.)
endif

compile:
	$(DOTNET_PATH) new console --force
	sed -i '/<\/PropertyGroup>/i \    <AllowUnsafeBlocks>true<\/AllowUnsafeBlocks>' obj/reverse-complement.csproj.nuget.g.props
	cp revcomp.csharpcore-8.csharpcore Program.cs
	$(DOTNET_PATH) build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(DOTNET_PATH) ./bin/Release/net7.0/reverse-complement.dll 0 < ../../../inputs/revcomp-input25000000.txt" CSharp reverse-complement $(NTIMES)

run:
	$(DOTNET_PATH) ./bin/Release/net7.0/reverse-complement.dll 0 < ../../../inputs/revcomp-input25000000.txt

mem:
	/usr/bin/time -v  $(DOTNET_PATH) ./bin/Release/net7.0/reverse-complement.dll 0 < ../../../inputs/revcomp-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes   $(DOTNET_PATH) ./bin/Release/net7.0/reverse-complement.dll 0 < ../../../inputs/revcomp-input25000000.txt

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj