# DB — Drizzle ORM (TypeScript) patterns

Use when the project is TypeScript/Next.js/Bun/Node and needs a type-safe SQL layer without Prisma's heavyweight engine process. Pairs with `stack-nextjs`. [E4 — expert assessment]

**Core versions:** `drizzle-orm` (latest on npm) + `drizzle-kit` (migrations CLI) as of 2026-04. Peer-deps: `pg` for Postgres, `better-sqlite3` / `@libsql/client` for SQLite, `mysql2` for MySQL. [UNVERIFIED: pin exact versions from npm before shipping]

**Schema-first, not code-first:**
```ts
// db/schema.ts
import { pgTable, serial, text, timestamp, integer } from "drizzle-orm/pg-core";

export const users = pgTable("users", {
  id: serial("id").primaryKey(),
  email: text("email").notNull().unique(),
  createdAt: timestamp("created_at").defaultNow().notNull(),
});

export const posts = pgTable("posts", {
  id: serial("id").primaryKey(),
  authorId: integer("author_id").references(() => users.id).notNull(),
  body: text("body").notNull(),
});
```
`schema.ts` IS the source of truth. All types flow from it — `typeof users.$inferSelect` gives you the row type.

**Query with full inference:**
```ts
import { eq } from "drizzle-orm";
const rows = await db.select().from(users).where(eq(users.id, 1));
// rows: { id: number; email: string; createdAt: Date }[]
```
No codegen step, no separate `.prisma` file. Type errors surface in the IDE immediately.

**Migrations via drizzle-kit:**
```bash
drizzle-kit generate     # diff schema.ts against prev snapshot → emit SQL in drizzle/
drizzle-kit migrate      # apply pending migrations
drizzle-kit studio       # local web UI to inspect data
```
Config in `drizzle.config.ts` — specify `dialect`, `schema`, `out`, `dbCredentials`.

**Connection / pool:**
```ts
import { drizzle } from "drizzle-orm/node-postgres";
import { Pool } from "pg";
const pool = new Pool({ connectionString: process.env.DATABASE_URL, max: 20 });
export const db = drizzle(pool, { schema });
```
Serverless (Vercel / CF Workers): use `neon-serverless` or `@libsql/client` driver instead — the `pg` Pool doesn't survive cold-start boundaries.

**Forbidden:** template-string SQL with untrusted input (`sql\`SELECT * WHERE x = ${userInput}\`` — use `sql.placeholder` or the query builder); committing `drizzle/meta/_journal.json` conflicts (merge manually or regenerate); mixing drizzle-kit versions across dev machines.
