all:
	usage="make pub - publish all subcrates"

pub:
	cd qtcore && cargo publish
	cd qtgui && cargo publish
	cd qtwidgets && cargo publish
