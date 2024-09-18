# build project docs
build-docs:
	RUSTDOCFLAGS="-Dwarnings -Arustdoc::private_intra_doc_links" cargo doc --no-deps --all-features --document-private-items
