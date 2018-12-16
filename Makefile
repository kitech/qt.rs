all:
	usage="make pub - publish all subcrates"

bdlib:
	# use less memory?
	cargo build -vv -j 1 --release

pub:
	cd qtcore && cargo publish
	cd qtgui && cargo publish
	cd qtwidgets && cargo publish
