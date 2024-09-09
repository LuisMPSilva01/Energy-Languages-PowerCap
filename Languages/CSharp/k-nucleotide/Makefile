NTIMES=20

export NTIMES

include ../../../config.env

ifndef DOTNET_PATH
$(error DOTNET_PATH is not set.)
endif

compile:
	$(DOTNET_PATH) new console --force
	cp knucleotide.csharpcore Program.cs
	$(DOTNET_PATH) add package Microsoft.Experimental.Collections --prerelease
	$(DOTNET_PATH) build -c Release --no-restore --no-self-contained

measure:
	sudo modprobe msr
	sudo ../../../RAPL/main "$(DOTNET_PATH) ./bin/Release/net7.0/k-nucleotide.dll 0 < ../../../inputs/knucleotide-input25000000.txt" CSharp k-nucleotide $(NTIMES)

run:
	$(DOTNET_PATH) ./bin/Release/net7.0/k-nucleotide.dll 0 < ../../../inputs/knucleotide-input25000000.txt

mem:
	/usr/bin/time -v $(DOTNET_PATH) ./bin/Release/net7.0/k-nucleotide.dll 0 < ../../../inputs/knucleotide-input25000000.txt

valgrind:
	valgrind --tool=massif --stacks=yes $(DOTNET_PATH) ./bin/Release/net7.0/k-nucleotide.dll 0 < ../../../inputs/knucleotide-input25000000.txt

clean:
	rm -f -r bin
	rm -f -r obj
	rm -f Program.cs
	rm -f *.csproj