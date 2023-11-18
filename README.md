# Trust but verify!

Alice maintains the Bobcoin cryptocurrency investment accounts of 2n clients. Alice diligently keeps
track of each account balance in her digital ledger, and accurately accounts for all Bobcoins under her
custody, the total sum of which is denoted at time t as a 64-bit unsigned integer B^t and is publicly known
by everyone. At every time t ≥ 0, Alice maintains an entry b(i)^t for each client i, where 0 ≤ i < 2^n. Bobcoin
has a maximum supply of 1,000,000,000 BOBC.
Alice hired Charlie to create cryptographic proofs of exclusive allotment for her ledger. Charlie designed
these proofs to work the same way as commitments. This means that Alice first has to commit to her
entire ledger at every time t, and publish the same commitment C^t and total funds B^t to every client to
convince them that her ledger is accurate.
Each client, upon receiving B^t, C^t, and d(i)^t, should be able to open the commitment to reveal b(i)^t in Alice’s
ledger, attaining certainty that (with overwhelming probability) this balance is exclusively allotted, and
was not double-counted by Alice to reach the published total B^t. In other words,
∀0 ≤ i < 2^n : verify(pp, B^t||b(i)^t, C^t, d(i)^t) → 2n∑i b(i)^t = B^t
