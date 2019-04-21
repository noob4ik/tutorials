FROM buildpack-deps:bionic-scm

RUN curl -L https://github.com/CraneStation/wasi-sdk/releases/download/wasi-sdk-4/wasi-sdk-4.0-linux-amd64.tar.gz | tar xz --strip-components=1 -C /

VOLUME /code
WORKDIR /code
CMD make
