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
	podman run --network=host -e AWS_DEFAULT_REGION=$AWS_DEFAULT_REGION -e AWS_SECRET_ACCESS_KEY=$AWS_SECRET_ACCESS_KEY -e AWS_ENDPOINT_URL=$AWS_ENDPOINT_URL -e AWS_ACCESS_KEY_ID=$AWS_ACCESS_KEY_ID quickwit:v0.8.2 run

# create QuickWit index from configuration yaml-file via REST API
create:
	cd experiments; curl -XPOST -H 'Content-Type: application/yaml' \
		'http://127.0.0.1:7280/api/v1/indexes' --data-binary \
		@schemaless_index_config.yaml

# search query to QuickWit via REST API
search:
	curl 'http://127.0.0.1:7280/api/v1/scanned-files/search?query=_type:file'

# run Calam filereceiver with  Podman
calamfr:
	podman run -p 3310:3310 --network=host calam:v0.1.0 fr --qwhost 127.0.0.1
