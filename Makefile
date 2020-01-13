BIN := buranko
VERBOSE_FLAG = $(if $(VERBOSE),-v)
GOBIN ?= $(shell go env GOPATH)/bin

.PHONY: all
all: clean build

testdeps:
	go get -d -t $(VERBOSE_FLAG)

test: testdeps
	go test $(VERBOSE_FLAG) ./...

.PHONY: test testdeps

.PHONY: lint
lint: $(GOBIN)/golint
	go vet ./...
	golint -set_exit_status ./...

$(GOBIN)/golint:
	cd && go get golang.org/x/lint/golint

.PHONY: build
build:
	go build -o $(BIN)

.PHONY: clean
clean:
	rm $(BIN)
	go clean
