## Deploying on Docker

```
docker build -t budi-server .
```

*Replace `<API_KEY>` with Supabase API key from the project setting.* 
```
docker run -dp 8080:8080 -e SUPABASE_API_KEY="<API_KEY>" budi-server
```


The server is now accessable via `localhost:8080`
