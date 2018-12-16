all:
	usage="make pub - publish all subcrates"

bdlib:
	# use less memory?
	cargo build -v -j 1 --release

pub:
	cd qtcore && cargo publish
	cd qtgui && cargo publish
	cd qtwidgets && cargo publish
