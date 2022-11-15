docker run -d \
        --name hello-chat-postgres \
        -e POSTGRES_PASSWORD=qwerty123 \                                                
        -p 5432:5432 postgres:alpine3.16