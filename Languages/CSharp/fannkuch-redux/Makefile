NTIMES=20

export NTIMES

include ../../../config.env

ifndef DOTNET_PATH
$(error DOTNET_PATH is not set.)
endif

compile:
	$(DOTNET_PATH) new console --force
	cp fannkuchredux.csharpcore-8.csharpcore Program.cs
	$(DOTNET_PATH) build -c Release --no-restore --no-self-contained 

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(DOTNET_PATH) ./bin/Release/net7.0/fannkuch-redux.dll 12" CSharp fannkuch-redux $(NTIMES)

run:
	$(DOTNET_PATH) ./bin/Release/net7.0/fannkuch-redux.dll 12

mem:
	/usr/bin/time -v  $(DOTNET_PATH) ./bin/Release/net7.0/fannkuch-redux.dll 12

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj
valgrind:
	valgrind --tool=massif --stacks=yes $(DOTNET_PATH) ./bin/Release/net7.0/fannkuch-redux.dll 12
