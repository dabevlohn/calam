# build calam
build:
	cargo build --release

# build project docs
build-docs:
	RUSTDOCFLAGS="-Dwarnings -Arustdoc::private_intra_doc_links" cargo doc --no-deps --all-features --document-private-items

# build OCI image with Dockerfile
build-image:
	buildah build

# run QuickWit with Podman
runindex:
	podman run -p 7280:7280 quickwit:v0.8.2 run

# create QuickWit index from configuration yaml-file via REST API
create:
	cd experiments; curl -XPOST -H 'Content-Type: application/yaml' \
		'http://192.168.1.68:7280/api/v1/indexes' --data-binary \
		@schemaless_index_config.yaml

# search query to QuickWit via REST API
search:
	curl 'http://192.168.1.68:7280/api/v1/scanned-files/search?query=_type:file'

# run Calam filereceiver with  Podman
calamfr:
	podman run -p 3310:3310 --network=host calam:v0.1.0 fr --qwhost 192.168.1.68
