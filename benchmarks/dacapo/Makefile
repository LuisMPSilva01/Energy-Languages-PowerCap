NTIMES=2
export NTIMES

problems ?= avrora batik

measure:
	@for prob in $(problems); do \
		sudo ../../RAPL/main "sudo /usr/lib/jvm/jdk-20/bin/java -jar dacapo-23.11-chopin.jar $$prob" dacapo $$prob $(NTIMES); \
	done

run:
	sudo /usr/lib/jvm/jdk-20/bin/java -jar dacapo-23.11-chopin.jar $(problems)

clean:
	echo "Cleaning..."
