Trying to reimagine v1 of tpuf. I also know little about Rust. The impetus of this project is that I'm interested in learning Creusot becasue formal methods / verification. Code written bespoke by yours truly. AI assisted for clarity, but not vibecoded as to entertain practice with Creusot.

In undergrad, I studied Formal Methods wrto model checking over transition systems. Solving problems usually involved computing policies that achieve two properties: reachability and unreachability. Reachability is when the system must reach a target state. Enforcing unreachbility means never reach a bad state. You can imagine that doing this might be useful in a game-theoretic setting, or for an autonomous robot. In finite worlds, you can do this naively by working backwards from target states. This means it even works in non-deterministic systems, but ultimately leads the designer to model the system faithfully, which is arguably harder.

For large state spaces, this naive method breaks down, but you at least know the properties exist. There are also infinite state systems, which we extend the thinking to Büchi Automata. In literature, we discuss the Büchi condition because Büchi automata operate on infinite words, so the reasoning applies to systems that do not terminate.

Creusot lives in another flavor of formal methods: deductive verification. Think of them as mathematical contracts about the code. 



As a corollary, I will use this proejct to take some field notes of what I think about Creusot/Rust.

1. quickly learned that creusot doesn’t have support for PartialOrd. It had a PR but it’s from 2023 and its latest comment was from 2025. So that was odd.

2. f32 giving me more trouble. it's hard to throw into a binary heap. Learned that making a wrapper struct for it was reasonable and not insane. I imagine rabitQ resolves this in v3. 

3. WAL time. The intent of the WAL is so pure and pristine.
