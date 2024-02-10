NTIMES=1

export NTIMES

compile:

measure:
	copy -r original_images temp_images
	sudo ../../../RAPL/main "optimize-images temp_images"  Python Optimize-images $(NTIMES)
run:
	copy -r original_images temp_images
	optimize-images temp_images

clean:
	rm -r temp_images