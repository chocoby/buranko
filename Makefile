VERBOSE_FLAG = $(if $(VERBOSE),-v)

testdeps:
	go get -d -t $(VERBOSE_FLAG)

test: testdeps
	go test $(VERBOSE_FLAG) ./...

.PHONY: test testdeps
