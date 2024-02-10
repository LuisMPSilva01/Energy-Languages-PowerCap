NTIMES=1

export NTIMES

compile:

measure:
	rm -f -r temp_images
	cp -r original_images temp_images
	sudo ../../RAPL/main "optimize-images temp_images"  Python Optimize-images $(NTIMES)
run:
	rm -f -r temp_images
	cp -r original_images temp_images
	optimize-images temp_images

clean:
	rm -f -r temp_images