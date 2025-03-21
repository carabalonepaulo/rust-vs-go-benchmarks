Todos os testes foram executados com os seguintes comandos:
`go build -ldflags="-s -w" -o .\main.exe`
`cargo run --release --quiet`

### Loop
go
```
Resultado: 7988005999000000000
Tempo total: 3.36s
```

rust
```
Resultado: 7988005999000000000
Tempo total: 0.000001200
```

### Create, insert 100000, select, delete e drop (UNFAIR)
go
```
Tempo total: 13.87s
```

rust
```
Tempo total: 16.647643800
```

### Select?
go (go-pgx)
```
Tempo total: 0.05 segundos
```

rust (tokio-pg)
```
Tempo total: 0.05 segundos
```

### Create, insert 100000, select, delete e drop
go
```
Tempo total: 13.12s
```

rust (parallel insert)
```
Tempo total: 3.661049200
```
