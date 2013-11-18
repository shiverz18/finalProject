all: hang

hang:
	rustc hang.rs

clean:
	rm -rf \#* *~ hang
