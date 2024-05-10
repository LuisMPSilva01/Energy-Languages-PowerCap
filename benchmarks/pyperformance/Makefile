NTIMES=2
export NTIMES

problems ?= 2to3 async_generators

measure:
    @for prob in $(problems); do \
        sudo ../../RAPL/main "sudo ~/.local/bin/pyperformance run -b $$prob" pyperformance $$prob $(NTIMES); \
    done

run:
    sudo ~/.local/bin/pyperformance run -b $(problem)

clean:
    echo "Cleaning..."

