#! /bin/bash

prefix="btt-"
cmd=$1
extra=$2

main() {
	options="edit-json\nwhiteout\ngen-ocr\nshow-labels\nadd-braille\nmonochrome\nview-out\nedit-out\nexit"

	action=$(echo -e $options | fzf)

	$action
}

edit-json() {
	json=$(test -z "$extra" && echo "./out.json" || echo "$extra")
	cargo run --bin "${prefix}edit-tools" "$json"

	main
}

whiteout() {
	json=$(test -z "$extra" && echo "./out.json" || echo "$extra")
	cargo run --bin "${prefix}whiteout-labels" ./diagram.png "$json"

	main
}

gen-ocr() {
	cargo run --bin "${prefix}get-ocr" ./diagram.png > out.json

	main
}

show-labels() {
	cargo run --bin "${prefix}label-ocr" ./diagram.png ./out.json

	main
}

add-braille() {
	json=$(test -z "$extra" && echo "./out.json" || echo "$extra")
	cargo run --bin "${prefix}add-braille" ./out.png "$json" $3

	main
}

monochrome() {
	convert diagram.png -monochrome mono.png
	echo "Exit SXIV to continue..."
	sxiv mono.png

	options="Keep\nDon't Keep"
	choice=$(echo -e $options | fzf)

	if ["$choice" == "Keep"];then
		convert diagram.png -monochrome diagram.png
		rm mono.png
	else
		rm mono.png
	fi

	main
}

view-out(){
	sxiv out.png &

	main
}

edit-out() {
	nvim out.json

	main
}

exit() {
	exit 0
}

main
