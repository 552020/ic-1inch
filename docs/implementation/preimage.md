# Preimage (Cryptographic Concept)

## Overview

A **preimage** is the original input data that, when passed through a cryptographic hash function, produces a specific hash output. In HTLC context, the preimage is the **secret** that generates the **hashlock**.

## What is a Preimage?

### **Basic Definition**

- **Preimage** = The original input to a hash function
- **Hash** = The output of the hash function
- **Relationship** = `hash = function(preimage)`

### **Simple Example**

```rust
// The preimage (original secret)
let preimage = b"my_secret_123";

// The hash function (SHA256)
let hash = sha256(preimage);
// Result: a1b2c3d4e5f6... (32 bytes)

// The preimage is what you need to know to "prove" you can generate this hash
```

## Preimage in HTLC Context

### **The HTLC Flow**

1. **Generate preimage** - Create a random secret (32 bytes)
2. **Create hashlock** - Hash the preimage: `hashlock = SHA256(preimage)`
3. **Store hashlock** - Put the hash in the escrow (not the preimage!)
4. **Reveal preimage** - To unlock funds, show the original secret

### **Why This Works**

```rust
// Step 1: Generate secret (preimage)
let secret = generate_random_secret();  // This is the preimage

// Step 2: Create hashlock
let hashlock = sha256(secret);  // Hash the preimage

// Step 3: Store only the hashlock (never the preimage!)
let escrow = Escrow {
    hashlock: hashlock,  // Only this is stored
    // secret is NOT stored anywhere!
};

// Step 4: To unlock, reveal the preimage
let revealed_secret = secret;  // The original preimage
let is_valid = verify_hashlock(revealed_secret, escrow.hashlock);
```

## Cryptographic Properties

### **One-Way Function**

- **Easy to compute**: `hash = SHA256(preimage)`
- **Hard to reverse**: Can't find `preimage` from `hash`
- **This is why it's secure** - You can't derive the secret from the hashlock

### **Preimage Resistance**

- **Given a hash**, it's computationally infeasible to find any preimage
- **This protects the secret** - Even if you see the hashlock, you can't find the secret

### **Second Preimage Resistance**

- **Given a preimage**, it's hard to find a different preimage that produces the same hash
- **This prevents forgery** - You can't create a fake secret that matches the hashlock

## Real-World Analogy

### **Think of it like a Lock and Key**

- **Preimage** = The key (the secret)
- **Hashlock** = The lock (the hash)
- **Verification** = Inserting the key to see if it opens the lock

### **The Security**

- **You can see the lock** (hashlock) - it's public
- **You can't see the key** (preimage) - it's secret
- **Only the key holder** can open the lock

## In Our ICP Implementation

### **Data Flow**

```rust
// 1. User generates preimage (secret)
let preimage = generate_random_secret();

// 2. User creates hashlock from preimage
let hashlock = sha256(preimage);

// 3. User submits order with hashlock (NOT preimage)
let order = CreateEscrowParams {
    hashlock: hashlock,  // Only the hash
    // preimage is kept secret!
};

// 4. Resolver creates escrow with same hashlock
let escrow = create_escrow(order);

// 5. User reveals preimage to claim funds
claim_escrow(escrow_id, preimage);  // Now reveal the secret
```

### **Security Guarantees**

- **Preimage is never stored** - Only exists in user's memory
- **Hashlock is public** - Can be seen by everyone
- **Verification is deterministic** - Same preimage always produces same hashlock

## Common Confusions

### **"What's the difference between preimage and hashlock?"**

- **Preimage** = The secret input (what you keep private)
- **Hashlock** = The hash output (what you make public)

### **"Why not just store the preimage?"**

- **Security risk** - If stored, anyone can steal funds
- **One-way property** - Hash can't be reversed to get preimage
- **Verification** - You prove you know the preimage by revealing it

### **"How do you know the preimage is correct?"**

- **Verification function** - `verify_hashlock(preimage, hashlock)`
- **Deterministic** - Same preimage always produces same hashlock
- **Cryptographic proof** - SHA256 is collision-resistant

## Testing Preimage Concepts

### **Generate and Verify**

```bash
# Generate a preimage (secret)
echo -n "my_secret_123" > preimage.txt

# Create hashlock from preimage
sha256sum preimage.txt
# Returns: a1b2c3d4e5f6...  preimage.txt

# The preimage is "my_secret_123"
# The hashlock is "a1b2c3d4e5f6..."
```

### **Test with Our Canister**

```bash
# Test preimage verification
dfx canister call backend test_hashlock '(
  blob "my_secret_123",    # The preimage
  blob "a1b2c3d4e5f6..."   # The hashlock
)'
# Should return: true
```

## Why This Matters for HTLC

### **Atomic Swap Security**

- **Same preimage** used on both chains
- **Same hashlock** ensures atomicity
- **Preimage revelation** unlocks both chains

### **1inch Fusion+ Integration**

- **User generates preimage** - Keeps it secret
- **User creates hashlock** - Submits with order
- **Resolver uses hashlock** - Creates escrows on both chains
- **User reveals preimage** - Unlocks destination escrow
- **Resolver uses same preimage** - Unlocks source escrow

---

**The preimage is the cryptographic secret that makes HTLC secure - it's the "key" that unlocks the "lock" (hashlock)!** 🔑
