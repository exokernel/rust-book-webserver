# Use an busybox latest as parent image
FROM busybox:latest

# Set the working directory to /app
WORKDIR /app

# Copy the current directory contents into the container at /app
ADD target/release/rust-book-webserver /app

# Make port 80 available to the world outside this container
EXPOSE 7878

# Run server.py when the container launches
CMD ["./rust-book-webserver"]
