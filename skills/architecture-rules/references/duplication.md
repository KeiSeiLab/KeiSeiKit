# Duplication Detection Rules

## Threshold Matrix

| Duplication Type | Threshold | Action |
|-----------------|-----------|--------|
| Exact copy (>10 lines) | 2 instances | Extract immediately |
| Similar structure (>70% match, >15 lines) | 3 instances | Extract to shared function |
| Similar structure (>70% match, 5-15 lines) | 4+ instances | Consider extraction |
| Config/boilerplate patterns | 5+ instances | Generate/template |
| 3 lines or fewer | Any count | Leave as is (3 lines < abstraction cost) |

## Detection Methods

### Method 1: Structural Grep
```bash
# Find duplicate function bodies (exact)
grep -rn "function_pattern" src/ | sort | uniq -d

# Find similar patterns with context
grep -rn "pattern" src/ --include="*.ts" -A 5
```

### Method 2: AST-Level Analysis
Look for these structural duplicates:
- Same `if/else` chain in multiple files
- Same error handling block repeated
- Same validation logic in different endpoints
- Same DB query with minor variations
- Same transformation pipeline

### Method 3: Git-Based Detection
```bash
# Files that always change together (shotgun surgery)
git log --name-only --pretty=format: | sort | uniq -c | sort -rn

# Similar commit patterns
git log --all --oneline | grep -i "fix.*same\|duplicate\|copy"
```

## Extraction Decision Tree

```
Found duplicate code?
│
├── Is it 3 lines or fewer?
│   └── YES → Leave it. Abstraction costs more than duplication.
│
├── Is it pure data/config?
│   └── YES → Extract to constants/config file
│
├── Is it identical across 2+ places?
│   ├── Same module → Extract to private helper
│   ├── Same layer → Extract to shared utility in that layer
│   └── Cross-layer → Extract to shared lib/package
│
├── Is it similar but not identical?
│   ├── Differs by 1-2 params → Parameterize (function args)
│   ├── Differs by behavior → Strategy pattern
│   ├── Differs by type → Generics / template
│   └── Differs significantly → Maybe NOT duplication (coincidence)
│
└── Is it boilerplate forced by framework?
    └── YES → Code generation, templates, or decorators
```

## What IS and ISN'T Duplication

### IS Duplication (extract):
- Same validation logic in multiple API endpoints
- Same error formatting in multiple catch blocks
- Same DB query pattern (find by X, include Y, map to Z)
- Same auth check repeated across routes
- Same DTO transformation in multiple services

### IS NOT Duplication (leave alone):
- Similar but domain-different logic (calculating tax vs calculating discount)
- Test setup code (each test should be self-contained)
- Interface implementations that look similar but serve different contracts
- 2-3 lines of glue code (creating instance, calling method, returning result)
- Logging statements (same format, different context)

## Refactoring Patterns for Duplication

### Pattern 1: Extract Function
```
BEFORE:
  // in file A
  const user = await db.user.findUnique({ where: { id }, include: { profile: true } });
  if (!user) throw new NotFoundException('User not found');

  // in file B (identical)
  const user = await db.user.findUnique({ where: { id }, include: { profile: true } });
  if (!user) throw new NotFoundException('User not found');

AFTER:
  // shared/users.ts
  async function findUserOrThrow(id: string) {
    const user = await db.user.findUnique({ where: { id }, include: { profile: true } });
    if (!user) throw new NotFoundException('User not found');
    return user;
  }
```

### Pattern 2: Parameterize
```
BEFORE:
  function getActiveUsers() { return db.user.findMany({ where: { status: 'active' } }); }
  function getPendingUsers() { return db.user.findMany({ where: { status: 'pending' } }); }

AFTER:
  function getUsersByStatus(status: UserStatus) {
    return db.user.findMany({ where: { status } });
  }
```

### Pattern 3: Strategy/Callback
```
BEFORE:
  function processCSV(data) { parse(data); validate(data); saveToS3(data); }
  function processJSON(data) { parse(data); validate(data); saveToDB(data); }

AFTER:
  function processData(data, parser, saver) {
    const parsed = parser(data);
    validate(parsed);
    saver(parsed);
  }
```

### Pattern 4: Template Method
```
BEFORE:
  class EmailNotifier { format() {...}  send() { format(); deliver(); log(); } }
  class SMSNotifier   { format() {...}  send() { format(); deliver(); log(); } }

AFTER:
  abstract class Notifier {
    abstract format(): string;
    abstract deliver(): void;
    send() { this.format(); this.deliver(); this.log(); }
  }
```

## Cross-Module Duplication Rules

### Where to Put Shared Code

```
project/
├── src/
│   ├── shared/           # Cross-module utilities
│   │   ├── utils/        # Pure functions (no deps)
│   │   ├── types/        # Shared type definitions
│   │   └── constants/    # Shared constants
│   ├── modules/
│   │   ├── users/        # Module-specific code
│   │   └── orders/       # Module-specific code
│   └── lib/              # Framework-specific shared code
│       ├── db.ts         # Database client
│       ├── redis.ts      # Redis client
│       └── logger.ts     # Logger config
```

### Monorepo Duplication
```
packages/
├── shared/               # Shared across all packages
│   ├── types/
│   ├── utils/
│   └── constants/
├── web/                  # Uses shared/
├── api/                  # Uses shared/
└── worker/               # Uses shared/
```

**Rule**: If 2+ packages duplicate the same logic, move to `shared/`.
**Exception**: Keep package-specific if the shared version would need too many conditionals.
