# PostgreSQL Docker Setup for BetterUptime

This setup provides a PostgreSQL database using Docker Compose for the BetterUptime project.

## Quick Start

1. **Start the PostgreSQL database:**
   ```bash
   docker-compose up -d postgres
   ```

2. **Start both PostgreSQL and PgAdmin:**
   ```bash
   docker-compose up -d
   ```

3. **Stop the services:**
   ```bash
   docker-compose down
   ```

## Services

### PostgreSQL Database
- **Host:** localhost
- **Port:** 5432
- **Database:** betteruptime
- **Username:** postgres
- **Password:** password
- **Connection String:** `postgresql://postgres:password@localhost:5432/betteruptime?schema=public`

### PgAdmin (Database Management UI)
- **URL:** http://localhost:8080
- **Email:** admin@betteruptime.com
- **Password:** admin

## Prisma Commands

After starting PostgreSQL, you can use these Prisma commands:

```bash
# Generate Prisma client
npx prisma generate

# Create and run migrations
npx prisma migrate dev --name init

# Reset database (development only)
npx prisma migrate reset

# View database in Prisma Studio
npx prisma studio

# Check migration status
npx prisma migrate status
```

## Environment Variables

Copy `.env.example` to `.env` and modify as needed:

```bash
cp .env.example .env
```

## Docker Commands

```bash
# View running containers
docker ps

# View logs
docker-compose logs postgres
docker-compose logs pgadmin

# Access PostgreSQL shell
docker exec -it betteruptime-postgres psql -U postgres -d betteruptime

# Backup database
docker exec betteruptime-postgres pg_dump -U postgres betteruptime > backup.sql

# Restore database
docker exec -i betteruptime-postgres psql -U postgres betteruptime < backup.sql
```

## Troubleshooting

### Port Already in Use
If port 5432 is already in use, change it in `docker-compose.yml`:
```yaml
ports:
  - "5433:5432"  # Use port 5433 instead
```

And update your `DATABASE_URL` accordingly.

### Reset Everything
To completely reset the database and volumes:
```bash
docker-compose down -v
docker-compose up -d
```
