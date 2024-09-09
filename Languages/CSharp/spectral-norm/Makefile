NTIMES=20

export NTIMES

include ../../../config.env

ifndef DOTNET_PATH
$(error DOTNET_PATH is not set.)
endif

compile:
	$(DOTNET_PATH) new console --force
	sed -i '/<\/PropertyGroup>/i \    <AllowUnsafeBlocks>true<\/AllowUnsafeBlocks>' obj/spectral-norm.csproj.nuget.g.props 
	cp spectralnorm.csharpcore-5.csharpcore Program.cs
	$(DOTNET_PATH) build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(DOTNET_PATH) ./bin/Release/net7.0/spectral-norm.dll 5500" CSharp spectral-norm $(NTIMES)

run:
	$(DOTNET_PATH) ./bin/Release/net7.0/spectral-norm.dll 5500

mem:
	/usr/bin/time -v  $(DOTNET_PATH) ./bin/Release/net7.0/spectral-norm.dll 5500

valgrind:
	valgrind --tool=massif --stacks=yes  $(DOTNET_PATH) ./bin/Release/net7.0/spectral-norm.dll 5500

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj