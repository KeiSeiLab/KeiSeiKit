# Design Patterns — When to Use

## Creational

### Factory Method
- **When**: Object creation depends on runtime conditions, need to decouple `new` from business logic
- **Signal**: `if/switch` choosing which class to instantiate
- **Implementation**: Abstract creator + concrete creators
- **Avoid when**: Only one product type, no variation expected

### Abstract Factory
- **When**: Families of related objects that must be used together (UI themes, DB dialects)
- **Signal**: Multiple factories that produce related objects
- **Avoid when**: Single family — use Factory Method instead

### Builder
- **When**: Object needs 4+ constructor params, optional fields, step-by-step construction
- **Signal**: Telescoping constructors, many optional params, config objects
- **Implementation**: Builder class with fluent API, `.build()` validates
- **Avoid when**: Simple object with 1-3 required params

### Singleton
- **When**: ONLY for true global state — DB connection pool, config, logger
- **Signal**: `getInstance()` pattern
- **DANGER**: Overused. Prefer dependency injection. Singletons hide dependencies and break tests
- **Modern alternative**: DI container (NestJS providers, Spring beans)

### Prototype
- **When**: Cloning complex objects is cheaper than creating from scratch
- **Signal**: Objects with heavy initialization, deep nested state
- **Avoid when**: Objects are simple to construct

## Structural

### Adapter
- **When**: Integrating incompatible interfaces (3rd party lib, legacy API)
- **Signal**: Wrapper that translates interface A to interface B
- **Rule**: Adapter should be thin — only translate, no business logic

### Facade
- **When**: Complex subsystem needs a simple entry point
- **Signal**: Client talks to 5+ classes to do one thing
- **Rule**: Facade doesn't prevent direct access to subsystem

### Decorator
- **When**: Add responsibilities dynamically without subclassing
- **Signal**: Need multiple optional behaviors that compose (logging + caching + auth)
- **Implementation**: Same interface as wrapped object
- **Modern**: Middleware chains (Express/NestJS), Python decorators, TypeScript decorators

### Proxy
- **When**: Control access, lazy loading, caching, logging
- **Signal**: Same interface but with intercepted behavior
- **Modern**: JS Proxy, ORM lazy loading, API rate limiting

### Composite
- **When**: Tree structures — UI components, file systems, org charts
- **Signal**: "Part-whole" hierarchy where leaf and composite treated same
- **Modern**: React component tree, AST nodes

## Behavioral

### Strategy
- **When**: Multiple algorithms interchangeable at runtime
- **Signal**: `if/else` or `switch` on algorithm type
- **Implementation**: Interface + concrete strategies + context
- **Modern**: Function params (pass algorithm as callback)

### Observer / Event Emitter
- **When**: One-to-many dependency, reactive updates
- **Signal**: "When X happens, notify Y, Z, W"
- **Implementation**: EventEmitter, pub/sub, webhooks
- **Modern**: RxJS, EventEmitter, Redis pub/sub, WebSocket broadcasts

### Command
- **When**: Undo/redo, queue operations, macro recording
- **Signal**: Need to parametrize actions, defer execution, support undo
- **Implementation**: Command object with execute() + undo()

### State Machine
- **When**: Object behavior changes based on internal state
- **Signal**: Multiple `if (state === 'X')` checks scattered in code
- **Implementation**: State interface + concrete states + transitions map
- **Modern**: XState, finite state machines, workflow engines

### Repository
- **When**: Abstract data access from business logic
- **Signal**: Business logic directly queries DB
- **Implementation**: Interface with CRUD + query methods, concrete per storage
- **Rule**: Repository returns domain objects, NOT raw DB rows

### Middleware / Chain of Responsibility
- **When**: Sequential processing with optional short-circuit
- **Signal**: Request passes through auth → validation → rate limit → handler
- **Modern**: Express/Koa middleware, NestJS guards/pipes/interceptors

### Dependency Injection
- **When**: ALWAYS for non-trivial apps
- **Signal**: Classes create their own dependencies with `new`
- **Implementation**: Constructor injection (preferred), setter injection
- **Modern**: NestJS modules, Spring, tsyringe, InversifyJS
- **Rule**: Depend on abstractions (interfaces), not concretions

## Architectural Patterns

### MVC / MVVM
- **When**: UI applications with clear data-view separation
- **Modern**: React (View) + Zustand/Redux (Model) + hooks/services (Controller)

### Clean Architecture / Hexagonal
- **When**: Business logic must be independent of framework/DB/UI
- **Layers**: Domain → Use Cases → Interface Adapters → Frameworks
- **Rule**: Dependencies point INWARD only

### CQRS
- **When**: Read and write patterns differ significantly
- **Signal**: Complex queries + simple writes, or read scaling needed
- **Avoid when**: Simple CRUD — overkill

### Event Sourcing
- **When**: Need full audit trail, time-travel debugging, complex domain events
- **Signal**: "What happened" matters more than "current state"
- **DANGER**: Massive complexity increase. Only if truly needed

### Microservices
- **When**: Independent deployment, team autonomy, different scaling needs
- **DANGER**: Overused. Start with monolith, extract when pain points emerge
- **Rule**: If you can't build a well-structured monolith, microservices won't help

## Selection Rules

```
Need to create objects?
├── Many optional params → Builder
├── Runtime type selection → Factory Method
├── Related object families → Abstract Factory
└── Expensive initialization → Prototype

Need to structure code?
├── Incompatible interface → Adapter
├── Simplify complex system → Facade
├── Add optional behavior → Decorator
├── Control access → Proxy
└── Tree structure → Composite

Need to manage behavior?
├── Multiple algorithms → Strategy
├── React to changes → Observer
├── Undo/redo needed → Command
├── State-dependent behavior → State Machine
├── Sequential processing → Middleware
└── Decouple creation from use → DI
```
