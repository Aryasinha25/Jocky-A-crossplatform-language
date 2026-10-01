FROM golang:1.21-alpine AS builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN go build -o /jocky-server ./cmd/jocky-server

FROM alpine:latest
WORKDIR /app
COPY --from=builder /jocky-server .
COPY migrations ./migrations
EXPOSE 8080
CMD ["/app/jocky-server"]
