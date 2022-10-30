## Deploying on Docker

1. Clone the repo
```
git clone https://github.com/de-timebank/server.git ./budi-backend/server
```
2. Enter the repo directory and switch to `development` branch
```
cd timebank-backend/server && git switch development
```
3. Build the Docker image
```
docker build -t budi-server .
```
4. Run the container

 
```dockerfile
# Replace <API_KEY> with the Supabase project API key
docker run -dp 8080:8080 -e SUPABASE_API_KEY="<API_KEY>" budi-server
```


The server is now accessable via `localhost:8080` 🎉
