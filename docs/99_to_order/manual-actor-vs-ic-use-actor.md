## Comparison: Manual Actor Setup vs. `ic-use-actor`

| Aspect                 | Manual Actor (`@dfinity/agent`)  | `ic-use-actor`                         |
| ---------------------- | -------------------------------- | -------------------------------------- |
| **Actor Creation**     | Manual creation in each context  | Single creation via `<ActorProvider>`  |
| **Agent Setup**        | Full control over `HttpAgent`    | Abstracted, with customization options |
| **Identity Injection** | Manual identity passing          | Automatic from context (SIWE, II)      |
| **React Integration**  | Not React-specific               | Designed for React with hooks          |
| **Type Safety**        | Optional manual typing           | Enforced with TypeScript generics      |
| **Interceptors**       | Manual implementation            | Built-in lifecycle hooks               |
| **Multiple Canisters** | Manual tracking                  | Nested providers per canister          |
| **Session Handling**   | Manual delegation handling       | Built-in error handling                |
| **Usage Pattern**      | `const actor = createActor(...)` | `const { actor } = useActor()`         |
| **Flexibility**        | Maximum control                  | Optimized for React dApps              |
| **Best For**           | Scripts, backends, non-React     | Modern React apps with identity flows  |

---

### Detailed Explanations

#### **Actor Creation**

- **Manual**: You must explicitly create actors using `Actor.createActor(idlFactory, { agent, canisterId })` in every component or context where you need them
- **ic-use-actor**: Actor is created once via `<ActorProvider>` and injected into the React tree through context, available everywhere via hooks

#### **Agent Setup**

- **Manual**: Full control over `HttpAgent` setup (headers, host, fetch implementations, custom networking)
- **ic-use-actor**: Agent setup is abstracted but allows custom agents to be injected for advanced use cases

#### **Identity Injection**

- **Manual**: Identity must be manually retrieved and passed to the agent (e.g., `authClient.getIdentity()`)
- **ic-use-actor**: Identity is automatically retrieved from context (e.g., from `ic-siwe-js` or Internet Identity) and passed to the actor

#### **React Integration**

- **Manual**: Not React-specific, requires custom hooks or context to share actor state across components
- **ic-use-actor**: Designed for React with `createUseActorHook` and context support for easy reuse

#### **Type Safety**

- **Manual**: Optional - you must manually ensure actor types match your `_SERVICE` interface
- **ic-use-actor**: Enforced with TypeScript generics - actor context and hooks are typed with the `_SERVICE` definition

#### **Interceptors**

- **Manual**: Interceptors must be implemented manually with wrappers around actor calls
- **ic-use-actor**: Built-in `onRequest`, `onResponse`, `onRequestError`, and `onResponseError` make it easy to hook into lifecycle events

#### **Multiple Canisters**

- **Manual**: Requires creating and tracking multiple actors manually, risk of duplication
- **ic-use-actor**: Can nest multiple `<ActorProvider>` components, each tied to different contexts (e.g., `useBackendActor`, `useOrdersActor`)

#### **Session Handling**

- **Manual**: Handling expired delegations or invalid sessions must be implemented manually
- **ic-use-actor**: Easily handled via `onResponseError` interceptor - can log users out and refresh UI if delegation expires

#### **Usage Pattern**

- **Manual**: Must import and instantiate actor or pass it down via props/context
- **ic-use-actor**: Actor accessed via hook: `const { actor } = useActor()` - no boilerplate needed

#### **Flexibility**

- **Manual**: Maximum control over internals (custom headers, advanced networking, low-level agent behavior)
- **ic-use-actor**: Less granular control but sufficient for 95% of React-based dApps, optimized for developer experience

#### **Recommended Use Cases**

- **Manual**: Low-level scripting, backends, or non-React projects requiring deep customization
- **ic-use-actor**: Modern React apps with identity-based flows (SIWE, Internet Identity), especially when working with multiple canisters

---

### Example Use Case Comparison

**Manual Pattern:**

```ts
import { Actor, HttpAgent } from "@dfinity/agent";
import { idlFactory, canisterId } from "../declarations/backend";
import { AuthClient } from "@dfinity/auth-client";

const authClient = await AuthClient.create();
await authClient.login({
  onSuccess: async () => {
    const identity = await authClient.getIdentity();
    const agent = new HttpAgent({ identity });
    await agent.fetchRootKey(); // only for local dev

    const actor = Actor.createActor(idlFactory, {
      agent,
      canisterId,
    });

    const result = await actor.my_method();
  },
});
```

**ic-use-actor Pattern:**

```tsx
// useActor.ts
const actorContext = createActorContext<_SERVICE>();
export const useActor = createUseActorHook<_SERVICE>(actorContext);

// Actors.tsx
<ActorProvider
  canisterId={canisterId}
  context={actorContext}
  identity={identity}
  idlFactory={idlFactory}
  onResponseError={(data) => {
    if (data.error.message.includes("Invalid delegation")) {
      clear();
      window.location.reload();
    }
  }}
>
  {children}
</ActorProvider>;

// AnyComponent.tsx
const { actor } = useActor();
useEffect(() => {
  actor.my_method().then(console.log).catch(console.error);
}, []);
```
