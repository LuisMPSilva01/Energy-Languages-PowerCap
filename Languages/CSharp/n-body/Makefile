NTIMES=20

export NTIMES

include ../../../config.env

ifndef DOTNET_PATH
$(error DOTNET_PATH is not set.)
endif

compile:
	$(DOTNET_PATH) new console --force
	cp nbody.csharpcore-9.csharpcore Program.cs
	$(DOTNET_PATH) build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(DOTNET_PATH) ./bin/Release/net7.0/n-body.dll 50000000" CSharp n-body $(NTIMES)

run:
	$(DOTNET_PATH) ./bin/Release/net7.0/n-body.dll 50000000

mem:
	/usr/bin/time -v  $(DOTNET_PATH) ./bin/Release/net7.0/n-body.dll 50000000

valgrind:
	valgrind --tool=massif --stacks=yes  $(DOTNET_PATH) ./bin/Release/net7.0/n-body.dll 50000000

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj