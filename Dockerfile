FROM arm64v8/alpine:3

COPY ./tmp/server /opt/commune

WORKDIR app

ENTRYPOINT ["/opt/commune"]
