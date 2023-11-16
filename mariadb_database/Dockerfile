# Use the official MariaDB image
FROM mariadb:jammy

# Set default environment variables
ENV MARIADB_USER=httpbin-user
ENV MARIADB_PASSWORD=aabb
ENV MARIADB_ROOT_PASSWORD=uuvv
ENV MARIADB_DATABASE=httpbin-db

# Copy the initialization script
COPY create_table.sql /docker-entrypoint-initdb.d/

# Expose the default MariaDB port
EXPOSE 3306
